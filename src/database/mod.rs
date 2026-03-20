#![allow(unused)]

use dioxus::prelude::{info, warn};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// const DATABASE_PATH: &str = "/srv/wedding/database.db";
const DATABASE_PATH: &str = "/home/kiewn/working/wasm-wedding-invitation/database.db";

#[derive(Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
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

pub fn query_user(uid: &str) -> Option<Person> {
    let conn = Connection::open(DATABASE_PATH)
        .inspect_err(|e| warn!("Database connection error: {e}"))
        .ok()?;

    let query_cmd = "SELECT p.name, p.greeting, p.line1, p.line2, p.line3, l.depart_from FROM people p JOIN location l on l.uid = p.uid WHERE p.uid = ?1";
    
    info!("Query: {}", query_cmd);
    info!("Params: uid='{}'", uid);

    conn.query_one(query_cmd, params![uid], |row| {
        let depart_location_id: i32 = row.get(5).unwrap_or_default();
        Ok(Person {
            uid: uid.to_string(),
            name: row.get(0)?,
            greeting: row.get(1)?,
            line1: row.get(2)?,
            line2: row.get(3)?,
            line3: row.get(4)?,
            depart_from: DepartLocation::try_from(depart_location_id).unwrap_or_default(),
        })
    })
    .inspect_err(|e| warn!("Database query error: {e}"))
    .ok()
}

pub fn update_location(uid: &str, location: DepartLocation) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    let query_cmd = "UPDATE location SET depart_from = ?1 WHERE uid = '?2'";
    info!("{}", query_cmd);

    let mut stmt = conn.prepare("UPDATE location SET depart_from = ?1 WHERE uid = ?2")?;
    stmt.execute(params![location as i32, uid])?;

    Ok(())
}
