use colored::*;
use fuujin::config;
use std::process;

fn main() {
    let options = config();

    if let Err(e) = fuujin::run(options) {
        eprintln!("Error: {}", e);

        process::exit(1);
    }

    println!("{}", "Component Created ".blue());
}
