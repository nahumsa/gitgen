use clap::{Parser, Subcommand};
use git2::Repository;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // diff
    Diff {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Diff {} => {
            let repo_path = ".";
            let repo = Repository::open(repo_path).expect("Failed to open repository");

            let mut revwalk = repo.revwalk().expect("Failed to initialize revwalk");
            revwalk.push_head().expect("Failed to push head");

            for oid in revwalk {
                let oid = oid.expect("Failed to get OID");
                let commit = repo.find_commit(oid).expect("Failed to find commit");

                println!(
                    "Commit: {} - {}",
                    oid,
                    commit.message().expect("unable to get message"),
                );
            }
        }
    };
}
