use clap::{Parser, Subcommand};
use githelp::git::run_git_diff;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // diff
    Diff {}
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Diff {} => {
            match run_git_diff() {
                Ok(output) => {
                if output.status.success() {
                    let diff_output = String::from_utf8_lossy(&output.stdout);
                    println!("Git diff output:\n{}", diff_output);
                } else {
                    eprintln!("Git diff command failed with an error: {:?}", output.status);
                    eprintln!("Error message: {:?}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(err) => {
                eprintln!("Failed to run git diff: {:?}", err);
            }
            }
         },
    };
}