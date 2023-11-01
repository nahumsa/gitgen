use std::process::{Command, Output};

pub fn run_git_diff() -> Result<Output, std::io::Error> {
    Command::new("git").arg("diff").output()
}

pub fn list_branches() -> Result<Output, std::io::Error> {
    Command::new("git").arg("branch").output()
}
