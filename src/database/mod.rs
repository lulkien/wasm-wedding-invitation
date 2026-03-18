use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Clone, Default, PartialEq)]
pub struct Message {
    pub line1: String,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub line4: Option<String>,
    pub line5: Option<String>,
}

impl Message {
    fn default_message() -> Message {
        Message { 
            line1: "My dearest friend".to_string(),
            line2: Some("The big day is officially happening, and you're one of the first people I wanted to tell.".to_string()),
            line3: Some("Would be so happy to have you by my side.".to_string()),
            ..Message::default()
        }
    }
}

pub type Database = HashMap<String, Message>;

static DB: OnceLock<RwLock<Database>> = OnceLock::new();

fn database_writer() -> Option<RwLockWriteGuard<'static, Database>> {
    DB.get_or_init(|| RwLock::new(Database::new()))
        .write()
        .map_err(|poison| poison.into_inner())
        .ok()
}

fn database_reader() -> Option<RwLockReadGuard<'static, Database>> {
    DB.get_or_init(|| RwLock::new(Database::new()))
        .read()
        .map_err(|poison| poison.into_inner())
        .ok()
}

pub fn query(id: &str) -> Message {
    let db = database_reader();

    if db.is_none() {
        return Message::default_message();
    }

    let db = db.unwrap();

    db.get(id).unwrap_or(&Message::default_message()).clone()
}

pub fn init_database() {
    let mut db = database_writer();

    if db.is_none() {
        return;
    }

    let mut db = db.unwrap();

    db.insert(
        "769a53cc".to_string(),
        Message {
            line1: "Yo, my boiz Bach".to_string(),
            line2: Some("It's kinda weird to say, but well... I'm getting married!".to_string()),
            line3: Some("Would love you to be there.".to_string()),
            ..Default::default()
        },
    );

    db.insert(
        "32985cff".to_string(),
        Message {
            line1: "Yo, my boiz Hoang,".to_string(),
            line2: Some("Guess what, I'm getting married after all...".to_string()),
            line3: Some("Would be awesome to have you there!".to_string()),
            ..Default::default()
        },
    );

    db.insert(
        "48fa6f6c".to_string(),
        Message {
            line1: "Yo Hoang,".to_string(),
            line2: Some("It's me! I'm getting married.".to_string()),
            line3: Some("Hope you'll be there to celebrate with us.".to_string()),
            ..Default::default()
        },
    );

    db.insert(
        "b814c11".to_string(),
        Message {
            line1: "Yo, Tuyet,".to_string(),
            line2: Some("I'm getting married! xD".to_string()),
            line3: Some("Let's come celebrate! :D".to_string()),
            ..Default::default()
        },
    );

}
