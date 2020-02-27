use clap::{Arg, App, SubCommand};

fn main() {
    let generate = SubCommand::with_name("generate")
        .about("Generate a new name")
        .arg(Arg::with_name("description")
            .help("Short description of your application"));

    let adjectives = SubCommand::with_name("adjectives")
        .about("Adjectives maintenance")
        .subcommand(SubCommand::with_name("list")
            .about("List current adjectives"))
        .subcommand(SubCommand::with_name("add")
            .about("Add a new adjective")
            .arg(Arg::with_name("adjective")
                .help("Adjective to be added")))
        .subcommand(SubCommand::with_name("rm")
            .about("Remove an adjective")
            .arg(Arg::with_name("adjective")
                .help("Adjective to be removed")));

    let metals = SubCommand::with_name("metals")
        .about("Metal names maintenance")
        .subcommand(SubCommand::with_name("list")
            .about("List current metal names"))
        .subcommand(SubCommand::with_name("add")
            .about("Add a new metal name")
            .arg(Arg::with_name("adjective")
                .help("Metal name to be added")))
        .subcommand(SubCommand::with_name("rm")
            .about("Remove a metal name")
            .arg(Arg::with_name("adjective")
                .help("Metal name to be removed")));

    let main = App::new("Name Rust Programs")
        .version("0.1")
        .author("Julio Biason <julio.biason@pm.me>")
        .about("From a short description, create a name for your application")
        .subcommand(generate)
        .subcommand(adjectives)
        .subcommand(metals);

    let matches = main.get_matches();
    dbg!(matches);
}
