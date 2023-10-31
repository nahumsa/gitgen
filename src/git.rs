use std::process::{Output, Command};

pub fn run_git_diff() -> Result<Output, std::io::Error> {
    Command::new("git").arg("diff").output()
}