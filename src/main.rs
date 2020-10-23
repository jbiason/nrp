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

fn main() {
    match args::parse() {
        Ok(x) => match x {
            actions::Action::AdjectiveList => {
                show_words(&repository::WordList::find_all_adjectives().unwrap());
            }
            actions::Action::AdjectiveAdd(word) => {
                repository::WordList::insert_adjective(&word).unwrap()
            }
            actions::Action::MetalList => {
                show_words(&repository::WordList::find_all_metals().unwrap())
            }
            actions::Action::MetalAdd(word) => repository::WordList::insert_metal(&word).unwrap(),
            _ => unimplemented!(),
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
