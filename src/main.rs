use std::{env, process};

use carapace::Config;


fn main() {
    println!("A rusty shell! 🦀");

    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = carapace::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
