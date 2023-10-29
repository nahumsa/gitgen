use clap::{command, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "gitgenius")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

enum Commands

fn main() {
    println!("Hello, world!");
}
