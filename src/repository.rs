use std::collections::BTreeMap;
use std::collections::LinkedList;
use std::fs::File;
use std::io::{Read, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use toml;

pub type WordStorage = BTreeMap<String, LinkedList<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    adjectives: WordStorage,
    metals: WordStorage,
}

#[derive(Debug)]
pub enum WordListError {
    InvalidFormat,
    InvalidWord,
    WordAlreadyExists,
    SaveFailure,
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

impl std::convert::From<toml::ser::Error> for WordListError {
    fn from(_error: toml::ser::Error) -> WordListError {
        WordListError::SaveFailure
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
    fn load() -> Result<Self, WordListError> {
        if let Ok(mut fp) = File::open("database.toml") {
            let mut content = String::new();
            fp.read_to_string(&mut content)?;
            let data = toml::from_str(&content)?;
            Ok(data)
        } else {
            Ok(WordList::empty())
        }
    }

    /// Save the database
    fn save(&self) -> Result<(), WordListError> {
        let content = toml::to_string(&self).unwrap();
        let mut fp = File::create("database.toml")?;
        fp.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Get the list of all adjectives
    pub fn find_all_adjectives() -> Result<WordStorage, WordListError> {
        let repo = Self::load()?;
        Ok(repo.adjectives)
    }

    /// Get the list of all metals
    pub fn find_all_metals() -> Result<WordStorage, WordListError> {
        let repo = Self::load()?;
        Ok(repo.metals)
    }

    /// Add an adjective to the word list
    pub fn insert_adjective(adjective: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::insert_word(adjective, &mut repo.adjectives)?;
        repo.save()
    }

    /// Add a metal to the word list
    pub fn insert_metal(metal: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::insert_word(metal, &mut repo.metals)?;
        repo.save()
    }

    /// Generic function to insert words in the storage; the target points in which of the lists
    /// the word should be inserted.
    fn insert_word(word: &str, target: &mut WordStorage) -> Result<(), WordListError> {
        let initial = word
            .chars()
            .nth(0)
            .ok_or(WordListError::InvalidWord)?
            .to_string()
            .to_lowercase();
        let mut list = if !target.contains_key(&initial) {
            let empty_list = LinkedList::new();
            empty_list
        } else {
            target.get(&initial).unwrap().to_owned()
        };

        if list.contains(&word.to_string()) {
            Err(WordListError::WordAlreadyExists)
        } else {
            list.push_back(word.to_string().to_lowercase());
            target.insert(initial, list);
            Ok(())
        }
    }
}
