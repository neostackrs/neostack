use clap::{Command, arg};

fn cli() -> Command {
    Command::new("neo-cli")
        .about("A command-line interface for NeoStackRS Static Site Generator")
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Create a new NeoStackRS project")
                .arg(arg!(<INPUT> "The input directory containing source files").required(true))
                .arg(arg!(--template -t [TEMPLATE] "The template to use for the new project").required(false))
        )
}

pub fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let input = sub_matches.get_one::<String>("INPUT").unwrap();
            let template = sub_matches.get_one::<String>("template").unwrap_or(&String::from("default"));
            println!("Input: {}", *input);
            println!("Template: {}", *template);
        }
        _ => unreachable!(), // clap will ensure we don't get here
    }
}