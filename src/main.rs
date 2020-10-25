/*
   NRP - Name Rust Program
   Copyright (C) 2020  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod actions;
mod args;
mod repository;

#[derive(Debug)]
enum Error {
    NoAdjectiveWith(String),
    NoMetalWith(String),
}

fn main() {
    match args::parse() {
        Ok(x) => match x {
            actions::Action::AdjectiveList => {
                show_words(&repository::WordList::find_all_adjectives().unwrap());
            }
            actions::Action::AdjectiveAdd(word) => {
                repository::WordList::insert_adjective(&word).unwrap()
            }
            actions::Action::AdjectiveRm(word) => {
                repository::WordList::remove_adjective(&word).unwrap()
            }
            actions::Action::MetalList => {
                show_words(&repository::WordList::find_all_metals().unwrap())
            }
            actions::Action::MetalAdd(word) => repository::WordList::insert_metal(&word).unwrap(),
            actions::Action::MetalRm(word) => repository::WordList::remove_metal(&word).unwrap(),
            actions::Action::Generate(description) => {
                println!("{}", generate(&description).unwrap());
            }
        },
        Err(x) => println!("Error {:?}", x),
    }
}

fn show_words(list: &repository::WordStorage) {
    let blank = String::from(" ");
    for (first_letter, word_list) in list {
        let heading = first_letter.to_uppercase();
        for (pos, word) in word_list.iter().enumerate() {
            println!("{}   {}", if pos == 0 { &heading } else { &blank }, word);
        }
    }
}

fn generate(description: &str) -> Result<String, Error> {
    let content = repository::WordList::load().unwrap();

    let initials = String::from(description)
        .trim()
        .split_whitespace()
        .map(|word| word.chars().nth(0).unwrap().to_string())
        .collect::<Vec<String>>();
    let number_of_initials = initials.len() - 1;

    let mut result = Vec::new();
    for (pos, letter) in initials.iter().enumerate() {
        result.push(if pos == number_of_initials {
            content
                .get_random_metal(&letter)
                .map_err(|_| Error::NoMetalWith(letter.to_string()))?
        } else {
            content
                .get_random_adjective(&letter)
                .map_err(|_| Error::NoAdjectiveWith(letter.to_string()))?
        });
    }
    Ok(result.join(" "))
}
