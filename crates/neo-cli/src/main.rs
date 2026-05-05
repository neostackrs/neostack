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
            println!("Creating a new NeoStackRS project...");
        }
        _ => unreachable!(), // clap will ensure we don't get here
    }
}