use std::fs;

fn main() {
    fs::copy("devly.toml", "target/debug/devly.toml").unwrap();
    fs::copy("devly.toml", "target/release/devly.toml").unwrap();
}