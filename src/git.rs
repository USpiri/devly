use std::process::Command;

pub fn get_changed_files() -> Vec<String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("failed to execute git status");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let files = clear_stdout(&stdout)
        .into_iter()
        .map(|line| line[2..].trim().to_string())
        .collect();
    files
}

pub fn get_staged_files() -> Vec<String> {
    let stdout = Command::new("git")
        .args(&["diff", "--name-only", "--cached"])
        .output()
        .expect("failed to execute git diff --name-only --cached");

    let output = String::from_utf8_lossy(&stdout.stdout).to_string();
    let lines = clear_stdout(&output);

    lines.iter().map(|line| line.to_string()).collect()
}

pub fn git_commit(commit: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .args(&["commit", "-m", &(commit)])
        .output()
        .map(|_| ())
}

pub fn git_add(files: &Vec<String>) -> Result<(), std::io::Error> {
    let mut command = Command::new("git");
    command.arg("add");
    for file in files {
        command.arg(file);
    }
    command.output().map(|_| ())
}

pub fn has_git_repository() -> bool {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .expect("failed to execute git command");

    output.status.success()
}

fn clear_stdout(commands: &str) -> Vec<&str> {
    commands
        .trim()
        .lines()
        .filter(|&line| !line.is_empty())
        .collect()
}

pub fn git_restore(files: &Vec<String>) -> Result<(), std::io::Error>  {
    let mut command = Command::new("git");
    command.arg("restore");
    for file in files {
        command.arg(file);
    }
    command.output().map(|_| ())

}