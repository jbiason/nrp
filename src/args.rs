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

use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::Arg;
// use clap::ArgMatches;
use clap::SubCommand;

#[derive(Debug)]
pub enum Action {
    Generate(String),
}

#[derive(Debug)]
pub enum ParseError {
    UnknownCommand,
    MissingDescription,
}

pub fn parse() -> Result<Action, ParseError> {
    let params = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate a name")
                .arg(
                    Arg::with_name("description")
                        .required(true)
                        .help("Short description of your application"),
                ),
        )
        .subcommand(
            SubCommand::with_name("adjectives")
                .about("Manage the adjective list")
                .subcommand(SubCommand::with_name("list").about("List current adjectives"))
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add an adjective")
                        .arg(Arg::with_name("adjective").help("Adjecive to be added")),
                )
                .subcommand(
                    SubCommand::with_name("rm")
                        .about("Remove an adjective")
                        .arg(Arg::with_name("adjective").help("Adjective to be removed")),
                ),
        )
        .subcommand(
            SubCommand::with_name("metals")
                .about("Manage the metal list")
                .subcommand(SubCommand::with_name("list").about("List all metals"))
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a metal to the list")
                        .arg(Arg::with_name("metal").help("Metal name to be added")),
                )
                .subcommand(
                    SubCommand::with_name("rm")
                        .about("Remove a metal from the list")
                        .arg(Arg::with_name("metal").help("Metal name to be removed")),
                ),
        );
    let matches = params.get_matches();
    match matches.subcommand() {
        ("generate", Some(arguments)) => {
            let description = arguments
                .value_of("description")
                .ok_or(ParseError::MissingDescription)?;
            Ok(Action::Generate(description.to_string()))
        }
        _ => Err(ParseError::UnknownCommand),
    }
}
