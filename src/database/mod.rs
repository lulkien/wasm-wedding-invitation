use std::{
    collections::HashMap, fmt::Display, sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard}
};

use dioxus::{fullstack::FromResponse, prelude::{info, warn}};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

const DATABASE_PATH: &str = "/srv/wedding/database.db";

#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
pub enum DepartLocation {
    #[default]
    None = 0,
    Fpt = 1,
    Lotte = 2,
    Nah = 3,
}

impl Display for DepartLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DepartLocation::None => write!(f, "I haven't decided yet"),
            DepartLocation::Fpt => write!(f, "I wanna depart from FPT Tower"),
            DepartLocation::Lotte => write!(f, "I wanna depart from Lotte"),
            DepartLocation::Nah => write!(f, "Sorry, I'll pass"),
        }
    }
}

impl TryFrom<i32> for DepartLocation {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Fpt),
            2 => Ok(Self::Lotte),
            3 => Ok(Self::Nah),
            _ => Err(())
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

impl Person {
    pub fn with_default_message() -> Person {
        Person { 
            greeting: "My dearest friend".to_string(),
            line1: "The big day is officially happening, and you're one of the first people I wanted to tell.".to_string(),
            line2: Some("Would be so happy to have you by my side.".to_string()),
            ..Person::default()
        }
    }
}

pub fn query_user(uid: &str) -> Person {
    info!("Query user: {}", uid);

    let conn = Connection::open(DATABASE_PATH);

    if let Err(e) = conn {
        warn!("Database connection error: {e}");
        return Person::with_default_message();
    }

    let query_cmd =
        "SELECT
            p.name,
            p.greeting,
            p.line1,
            p.line2,
            p.line3,
            l.depart_from
        FROM people p 
        JOIN location l on l.uid = p.uid
        WHERE p.uid = ?1";

     conn.unwrap().query_one(
        query_cmd,
        [uid],
        |row | {
            let depart_location_id: i32 = row.get(5).unwrap_or_default();

            info!("{depart_location_id}");

            Ok(Person {
                uid: uid.to_string(),
                name: row.get(0)?,
                greeting: row.get(1)?,
                line1: row.get(2)?,
                line2: row.get(3)?,
                line3: row.get(4)?,
                depart_from: DepartLocation::try_from(depart_location_id).unwrap_or_default(),
            })
        }).inspect_err(|e| {
            warn!("Database query error: {e}")
        }).unwrap_or(Person::with_default_message())

}

pub fn update_location(uid: &str, location: DepartLocation) -> Result<()> {
    info!("Update depart location for user: {}. {}", uid, location);

    let conn = Connection::open(DATABASE_PATH)?;

    let mut stmt = conn.prepare("UPDATE location SET depart_from = ?1 WHERE uid = ?2")?;
    stmt.execute(params![location as i32, uid])?;

    Ok(())
}
