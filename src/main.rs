use std::{env, fs};


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config{query: args[1].clone(), file_path: args[2].clone() }
    }
}

fn main() {
    println!("A rusty shell! 🦀");

    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents: String = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
