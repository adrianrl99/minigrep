mod lib;

use std::env;
use std::process;

fn show_help() {
    println!("{} {}", env::args().nth(0).expect("minigrep"), "-Like grep");
    // Arguments
    // Optional
    println!();
    println!("{}", "Optional Argumets:");
    println!(" {}    {}", "--help", "-Show help");
    // Required
    println!();
    println!("{}", "Required Arguments:");
    println!(" {}    {}", "text", "-Text to search");
    println!(" {}    {}", "file", "-File to search");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Show  the help if the first argument is --help
    if args.len() == 2 {
        if args[1] == "--help" {
            show_help();
        } 
        process::exit(0);
    }

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}. {}", err, "Use --help for help");
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
