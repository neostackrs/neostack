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

#[derive(Parser, Subcommand)]
pub enum Commands {}

pub fn main() {}