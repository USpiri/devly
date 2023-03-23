use std::fs;

fn main() {
    fs::copy("devly_commits.toml", "target/debug/devly_commits.toml").unwrap();
    fs::copy("devly_commits.toml", "target/release/devly_commits.toml").unwrap();
}