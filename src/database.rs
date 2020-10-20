use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use toml;

type WordStorage = BTreeMap<char, LinkedList<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    adjectives: WordStorage,
    metals: WordStorage,
}

pub enum DatabaseError {
    InvalidFormat,
}

impl std::convert::From<std::io::Error> for DatabaseError {
    fn from(_error: std::io::Error) -> DatabaseError {
        DatabaseError::InvalidFormat
    }
}

impl std::convert::From<toml::de::Error> for DatabaseError {
    fn from(_error: toml::de::Error) -> DatabaseError {
        DatabaseError::InvalidFormat
    }
}

impl Database {
    /// Create an empty database
    fn empty() -> Self {
        Database {
            adjectives: WordStorage::new(),
            metals: WordStorage::new(),
        }
    }

    /// Load the database
    pub fn load() -> Result<Self, DatabaseError> {
        if let Ok(mut fp) = File::open("database.toml") {
            let mut content = String::new();
            fp.read_to_string(&mut content)?;
            let data = toml::from_str(&content)?;
            Ok(data)
        } else {
            Ok(Database::empty())
        }
    }
}
