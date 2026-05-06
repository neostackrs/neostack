use clap::{Command, arg};

fn global_args() -> [clap::Arg; 7] {
    [
        arg!(--verbose -V "Increase output verbosity").global(true),
        arg!(--quiet -q "Decrease output verbosity").global(true),
        arg!(--config -c [CONFIG] "The config file to use").global(true),
        arg!(--output -o [OUTPUT] "The output directory").global(true),
        arg!(--dest -d [DEST] "The destination directory").global(true),
        arg!(--watch -w "Watch for changes and rebuild").global(true),
    ]
}

fn convert_commands() -> [Command; 3] {
    [
        Command::new("to-json")
            .about("Convert frontmatter to JSON format")
            .arg_required_else_help(true)
            .args(global_args()),
        Command::new("to-yaml")
            .about("Convert frontmatter to YAML format")
            .arg_required_else_help(true)
            .args(global_args()),
        Command::new("to-toml")
            .about("Convert frontmatter to TOML format")
            .arg_required_else_help(true)
            .args(global_args()),
    ]
}

fn cli() -> Command {
    Command::new("neo")
        .about("A command-line interface for NeoStackRS Static Site Generator")
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .args(global_args())
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("new")
                .about("Create a new NeoStackRS project")
                .alias("create").alias("n")
                .arg_required_else_help(true)
                .args(global_args())
                .arg(arg!(<INPUT> "The input directory containing source files").required(true))
                .arg(arg!(--template -t [TEMPLATE] "The template to use for the new project").required(false))
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new NeoStackRS project")
                .alias("i")
                .arg_required_else_help(true)
                .args(global_args())
        )
        .subcommand(
            Command::new("build")
                .about("Build the static site from the source files")
                .alias("b")
                .arg_required_else_help(true)
                .args(global_args())
        )
        .subcommand(
            Command::new("server")
                .about("Start a development server for the static site")
                .alias("serve").alias("s")
                .arg_required_else_help(true)
                .args(global_args())
                .arg(arg!(--host -H [HOSTNAME] "The host to listen on").required(false))
                .arg(arg!(--port -p [PORT] "The port to listen on").required(false).default_value("6737"))
                
        )
        .subcommand(
            Command::new("convert")
                .about("Convert frontmatter between different formats")
                .arg_required_else_help(true)
                .args(global_args())
                .subcommands(convert_commands())

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