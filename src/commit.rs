use inquire::{
    formatter::OptionFormatter, max_length, min_length, required, validator::Validation, Confirm,
    MultiSelect, Select, Text,
};

use crate::{
    commit_types::{load_commit_types, CommitType},
    git::{get_changed_files, get_staged_files, git_add, git_commit, has_git_repository},
    utils::{print_err, print_success},
};

pub fn commit() {
    if !has_git_repository() {
        print_err("Not a git repository");
        std::process::exit(1);
    }
    if get_changed_files().is_empty() {
        print_err("Not files to commit");
        std::process::exit(1);
    }
    let commit_type = match select_commit_type() {
        Ok(commit) => commit,
        Err(e) => {
            print_err(e);
            std::process::exit(1);
        }
    };
    let mut breaking_change = false;
    if commit_type.release {
        breaking_change = ask_for_braking_change();
    }
    let commit_message = match ask_for_git_message() {
        Ok(msg) => msg,
        Err(e) => {
            print_err(e);
            std::process::exit(1);
        }
    };

    let commit = if commit_type.name == "Initial Commit" {
        format!("{} {}", commit_type.emoji, commit_type.name)
    } else if breaking_change {
        format!(
            "{} {}: {} \n\n{}\n",
            commit_type.emoji, commit_type.name, commit_message, "[BREAKING CHANGE]"
        )
    } else {
        format!(
            "{} {}: {}",
            commit_type.emoji, commit_type.name, commit_message
        )
    };
    ask_to_add_files();

    if Confirm::new(&format!("Confirm to commit?\n{commit}\n"))
        .with_default(true)
        .prompt()
        .unwrap()
    {
        match git_commit(&commit) {
            Ok(_) => print_success(&format!("Commited: {}", commit)),
            Err(e) => print_err(&e.to_string()),
        };
    };
}

fn ask_for_git_message() -> Result<String, &'static str> {
    let validator = |a: &str| {
        if a.trim().len() == 0 {
            return Ok(Validation::Invalid(
                "Commit message must not be empty".into(),
            ));
        }
        Ok(Validation::Valid)
    };

    let message = Text::new("Commit message:")
        .with_validator(required!())
        .with_validator(min_length!(1))
        .with_validator(max_length!(150))
        .with_validator(validator)
        .prompt()
        .map_err(|_| "Unable to set commit message or canceled commit")?;

    Ok(message.trim().to_owned())
}

fn ask_for_braking_change() -> bool {
    let ans = Confirm::new("Does the commit contain a breaking change?")
        .with_default(false)
        .with_help_message(
            "Your response may trigger a new MAJOR version if it's a breaking change.",
        )
        .prompt();
    match ans {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(_) => std::process::exit(1),
    }
}

fn select_commit_type() -> Result<CommitType, &'static str> {
    let commits: Vec<CommitType> = match load_commit_types() {
        Ok(commit_types) => commit_types,
        Err(e) => {
            eprintln!("Error loading commit types: {}", e);
            std::process::exit(1);
        }
    };
    let formatter: OptionFormatter<CommitType> = &|commit| {
        format!(
            "\n{} {}: {}",
            commit.value.emoji, commit.value.name, commit.value.description
        )
    };
    let ans = Select::new("Select commit type:", commits)
        .with_formatter(formatter)
        .prompt()
        .map_err(|_| "Error selecting commit type");

    match ans {
        Ok(commit_type) => Ok(commit_type),
        Err(e) => return Err(e),
    }
}

fn ask_to_add_files() {
    if !get_staged_files().is_empty() {
        println!("You have some files in stage to be commited...");
        println!("{:?}", get_staged_files());
        let ans = Confirm::new("Do you want to proceed with staged files (yes) or add new files?")
            .with_default(true)
            .with_help_message(
                "If you does'n want to add these files, please 'ESC' and type 'git reset <files>'",
            )
            .prompt();
        match ans {
            Ok(false) => {
                let mut options = get_changed_files();
                let staged = get_staged_files();
                options.retain(|x| !staged.contains(&x));
                add_files(options);
                return;
            }
            Ok(_) => (),
            Err(_) => std::process::exit(1),
        }
    }
    let options = get_changed_files();
    add_files(options);
}

fn add_files(options: Vec<String>) {
    let files_vec = MultiSelect::new("Select the files you want to add:", options)
        .prompt()
        .unwrap();
    if files_vec.is_empty() && get_staged_files().is_empty() {
        print_err("No files found to add");
        std::process::exit(1);
    }
    match git_add(&files_vec) {
        Ok(_) => print_success("Added Files"),
        Err(_) => print_err("Unable to add files, please try again later"),
    };
}
