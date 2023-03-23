use console::style;

pub fn print_err(msg:&str){
    println!("{} {}", style("Error:").red().bold(), msg);
}

pub fn print_success(msg:&str){
    println!("{} {}", style("Success!").green().bold(), msg);
}