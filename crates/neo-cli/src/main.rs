use clap::{Command, arg};

fn cli() -> Command {
    Command::new("neo")
        .about("A command-line interface for NeoStackRS Static Site Generator")
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Create a new NeoStackRS project")
                .alias("create").alias("n")
                .arg(arg!(<INPUT> "The input directory containing source files").required(true))
                .arg(arg!(--template -t [TEMPLATE] "The template to use for the new project").required(false))
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new NeoStackRS project")
                .alias("i")
        )
        .subcommand(
            Command::new("build")
                .about("Build the static site from the source files")
                .alias("b")
        )
        .subcommand(
            Command::new("server")
                .about("Start a development server for the static site")
                .alias("serve").alias("s")
        )
}

pub fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", _sub_matches)) => {
            println!("Creating a new NeoStackRS project...");
        }
        Some(("init", _sub_matches)) => {
            println!("Initializing a new NeoStackRS project...");
        }
        Some(("build", _sub_matches)) => {
            println!("Building the static site...");
        }
        Some(("server", _sub_matches)) => {
            println!("Starting the development server...");
        }
        _ => unreachable!(), // clap will ensure we don't get here
    }
}