extern crate git2;

use git2::Repository;

fn main() {
    println!("Starting Git Cli");

    let repo = match Repository::init(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
    println!("Repo State: {:?}", repo.state());
}
