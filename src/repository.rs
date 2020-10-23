use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Read, Write};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use toml;

pub type WordStorage = BTreeMap<String, BTreeSet<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WordList {
    adjectives: WordStorage,
    metals: WordStorage,
}

#[derive(Debug)]
pub enum WordListError {
    InvalidFormat,
    InvalidWord,
    SaveFailure,
    NoSuchWord,
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

    /// Add an adjective to the word list
    pub fn insert_adjective(adjective: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::insert_word(adjective, &mut repo.adjectives)?;
        repo.save()
    }

    /// Remove an adjective
    pub fn remove_adjective(adjective: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::remove_word(adjective, &mut repo.adjectives)?;
        repo.save()
    }

    /// Get the list of all metals
    pub fn find_all_metals() -> Result<WordStorage, WordListError> {
        let repo = Self::load()?;
        Ok(repo.metals)
    }

    /// Add a metal to the word list
    pub fn insert_metal(metal: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::insert_word(metal, &mut repo.metals)?;
        repo.save()
    }

    /// Remove a metal
    pub fn remove_metal(metal: &str) -> Result<(), WordListError> {
        let mut repo = Self::load()?;
        Self::remove_word(metal, &mut repo.metals)?;
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
            let empty_list = BTreeSet::new();
            empty_list
        } else {
            target.get(&initial).unwrap().to_owned()
        };

        list.insert(word.to_string().to_lowercase());
        target.insert(initial, list);
        Ok(())
    }

    /// Generic function to remove a word from the storage; follows the same logic as insert_word.
    fn remove_word(word: &str, target: &mut WordStorage) -> Result<(), WordListError> {
        let initial = word
            .chars()
            .nth(0)
            .ok_or(WordListError::InvalidWord)?
            .to_string()
            .to_lowercase();
        let the_word = word.to_string().to_lowercase();
        let list = target.get_mut(&initial).ok_or(WordListError::NoSuchWord)?;
        list.remove(&the_word);
        if list.is_empty() {
            target.remove(&initial);
        }
        Ok(())
    }
}
