use inquire::MultiSelect;

use crate::{
    git::{get_changed_files, has_git_repository, git_restore},
    utils::{print_err, print_success},
};

pub fn restore() {
    if !has_git_repository() {
        print_err("Not a git repository");
        std::process::exit(1);
    }
    restore_files();
}

fn restore_files() {
    let options = get_changed_files();
    let files_to_restore = MultiSelect::new("Select the files you want to restore", options)
        .prompt()
        .unwrap();
    if files_to_restore.is_empty() {
        print_err("No files found to restore");
        std::process::exit(1);
    }
    match git_restore(&files_to_restore) {
        Ok(_) => print_success("Restored Files"),
        Err(_) => print_err("Unable to restore files, please try again later"),
    }
}