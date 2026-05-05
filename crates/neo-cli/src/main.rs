use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neo")]
#[command(version)]
#[command(about = "NeoStack Static Site Generator", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short= "V", long, global = true)]
    pub verbose: bool,
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {

    #[command(alias = "create")]
    #[command(alias = "n")]
    New {
        name: String

        #[arg(short, long, default_value = "default")]
        template: Option<String>
    },
    #[command(alias = "i")]
    Init,
    #[command(alias = "a")]
    Add {
        package: String
        version: Option<String> 
    },
    #[command(alias = "rm")]
    Remove { name: String },
    #[command(alias = "b")]
    Build,
    #[command(alias = "c")]
    Clean,
    Config {
        #[command(subcommand)]
        command: ConfigCommands
    },
    #[command(alias = "d")]
    Doctor,
    #[command(alias = "s")]
    Serve {
        #[arg(short, long, default_value_t = 4321)]
        port: u16

        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        #[arg(long)]
        no_watch: bool,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(alias = "s")]
    Set { key: String, value: String },
    
    #[command(alias = "g")]
    Get { key: String },
    
    #[command(alias = "d")]
    Delete { key: String },

    #[command(alias = "sh")]
    #[command(alias = "list")]
    #[command(alias = "ls")]
    Show,
}