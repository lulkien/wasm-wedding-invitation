#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::fmt::Display;

// ── Shared types (compiled for both client and server) ────────────────────────

#[derive(Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
pub enum DepartLocation {
    #[default]
    None = 0,
    Fpt = 1,
    Handico = 2,
    Lotte = 3,
    MyVehicle = 4,
    Nah = 5,
}

impl Display for DepartLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepartLocation::None => write!(f, "I haven't decided yet"),
            DepartLocation::Fpt => write!(f, "I wanna depart from FPT Tower"),
            DepartLocation::Handico => write!(f, "I wanna depart from Handico Tower"),
            DepartLocation::Lotte => write!(f, "I wanna depart from Lotte"),
            DepartLocation::MyVehicle => write!(f, "I will use my own vehicle"),
            DepartLocation::Nah => write!(f, "Sorry, I'll pass"),
        }
    }
}

impl TryFrom<i64> for DepartLocation {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Fpt),
            2 => Ok(Self::Handico),
            3 => Ok(Self::Lotte),
            4 => Ok(Self::MyVehicle),
            5 => Ok(Self::Nah),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Person {
    pub uid: String,
    pub name: String,
    pub greeting: String,
    pub line1: String,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub depart_from: DepartLocation,
}

// ── Server-only database implementation ──────────────────────────────────────

#[cfg(feature = "server")]
mod server {
    use super::*;
    use dioxus::prelude::{info, warn};
    use std::sync::LazyLock;
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        types::SurrealValue,
        Surreal,
    };
    use tokio::sync::OnceCell;


    #[cfg(debug_assertions)]
    const DB_URL: &str = "ws://127.0.0.1:8000";

    #[cfg(not(debug_assertions))]
    const DB_URL: &str = "ws://127.0.0.1:8000";

    const DB_NS: &str = "wedding";
    const DB_NAME: &str = "wedding";
    const DB_USER: &str = "root";
    const DB_PASS: &str = "secret";

    static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
    static CONNECTED: OnceCell<()> = OnceCell::const_new();

    async fn get_db() -> &'static Surreal<Client> {
        CONNECTED
            .get_or_init(|| async {
                DB.connect::<Ws>(DB_URL)
                    .await
                    .expect("Failed to connect to SurrealDB");
                DB.signin(Root {
                    username: DB_USER.to_string(),
                    password: DB_PASS.to_string(),
                })
                .await
                .expect("Failed to sign in to SurrealDB");
                DB.use_ns(DB_NS)
                    .use_db(DB_NAME)
                    .await
                    .expect("Failed to select SurrealDB namespace/database");
            })
            .await;
        &DB
    }

    fn is_valid_uid(uid: &str) -> bool {
        uid.len() == 8 && uid.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Row returned by SurrealDB — depart_from is stored as an integer.
    #[derive(SurrealValue)]
    struct PersonRow {
        uid: String,
        name: String,
        greeting: String,
        line1: String,
        line2: Option<String>,
        line3: Option<String>,
        depart_from: i64,
    }

    pub async fn query_user(uid: &str) -> Option<Person> {
        if !is_valid_uid(uid) {
            warn!("Invalid uid: '{uid}'");
            return None;
        }

        let db = get_db().await;

        let mut response = db
            .query(
                "SELECT uid, name, greeting, line1, line2, line3, depart_from \
                 FROM person WHERE uid = $uid",
            )
            .bind(("uid", uid.to_string()))
            .await
            .inspect_err(|e| warn!("SurrealDB query error: {e}"))
            .ok()?;

        let row: Option<PersonRow> = response
            .take(0)
            .inspect_err(|e| warn!("SurrealDB deserialize error: {e}"))
            .ok()?;

        row.map(|r| Person {
            uid: r.uid,
            name: r.name,
            greeting: r.greeting,
            line1: r.line1,
            line2: r.line2,
            line3: r.line3,
            depart_from: DepartLocation::try_from(r.depart_from).unwrap_or_default(),
        })
    }

    pub async fn update_location(uid: &str, location: DepartLocation) -> surrealdb::Result<()> {
        if !is_valid_uid(uid) {
            warn!("Invalid uid: '{uid}'");
            return Ok(());
        }

        let db = get_db().await;

        db.query("UPDATE person SET depart_from = $depart_from WHERE uid = $uid")
            .bind(("uid", uid.to_string()))
            .bind(("depart_from", location as i64))
            .await?;

        info!("Updated depart_from for uid={uid} to {}", location as i64);
        Ok(())
    }
}

#[cfg(feature = "server")]
pub use server::{query_user, update_location};
