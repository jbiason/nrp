use clap::{Arg, ArgMatches, App, SubCommand};

fn main() {
    let generate = SubCommand::with_name("generate")
        .about("Generate a new name")
        .arg(Arg::with_name("description")
            .required(true)
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
            .arg(Arg::with_name("name")
                .help("Metal name to be added")))
        .subcommand(SubCommand::with_name("rm")
            .about("Remove a metal name")
            .arg(Arg::with_name("name")
                .help("Metal name to be removed")));

    let main = App::new("Name Rust Programs")
        .version("0.1")
        .author("Julio Biason <julio.biason@pm.me>")
        .about("From a short description, create a name for your application")
        .subcommand(generate)
        .subcommand(adjectives)
        .subcommand(metals);

    let matches = main.get_matches();
    match matches.subcommand() {
        ("metals", Some(metal_command))   => metals_commands(&metal_command),
        ("adjectives", Some(adj_command)) => adjective_commands(&adj_command),
        ("generate", Some(gen_command))   => generate_command(&gen_command),
        (_, _) => panic!("I don't know how to handle that!"),
    };
}

fn metals_commands(command: &ArgMatches) {
    match command.subcommand() {
        ("list", Some(_))      => metals_list(),
        ("add", Some(add_cmd)) => metals_add(add_cmd.value_of("name")),
        ("rm", Some(rm_cmd))   => metals_rm(rm_cmd.value_of("name")),
        (command, _)           => println!("Invalid command: {}", command),
    }
}

fn adjective_commands(command: &ArgMatches) {
    match command.subcommand() {
        ("list" , Some(_))       => adjective_list(),
        ("add"  , Some(add_cmd)) => adjective_add(add_cmd.value_of("name")),
        ("rm"   , Some(rm_cmd))  => adjective_rm(rm_cmd.value_of("name")),
        (command, _)             => println!("Invalid command: {}", command),
    }
}

fn generate_command(command: &ArgMatches) {
    let app_description = command.value_of("description");
    dbg!("generate description", app_description);
}

fn metals_list() {
    dbg!("list metals");
}

fn metals_add(name: Option<&str>) {
    dbg!("Add metal", name);
}

fn metals_rm(name: Option<&str>) {
    dbg!("Rmove metal", name);
}

fn adjective_list() {
    dbg!("list adjectives");
}

fn adjective_add(name: Option<&str>) {
    dbg!("Add adjective", name);
}

fn adjective_rm(name: Option<&str>) {
    dbg!("Remove adjective", name);
}
