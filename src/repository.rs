use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use toml;

type WordStorage = BTreeMap<char, LinkedList<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    adjectives: WordStorage,
    metals: WordStorage,
}

pub enum WordListError {
    InvalidFormat,
}

impl std::convert::From<std::io::Error> for WordListError {
    fn from(_error: std::io::Error) -> WordListError {
        WordListError::InvalidFormat
    }
}

impl std::convert::From<toml::de::Error> for WordListError {
    fn from(_error: toml::de::Error) -> WordListError {
        WordListError::InvalidFormat
    }
}

impl WordList {
    /// Create an empty database
    fn empty() -> Self {
        WordList {
            adjectives: WordStorage::new(),
            metals: WordStorage::new(),
        }
    }

    /// Load the database
    pub fn load() -> Result<Self, WordListError> {
        if let Ok(mut fp) = File::open("database.toml") {
            let mut content = String::new();
            fp.read_to_string(&mut content)?;
            let data = toml::from_str(&content)?;
            Ok(data)
        } else {
            Ok(WordList::empty())
        }
    }
}
