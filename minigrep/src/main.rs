use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("An error occurred: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("An error occurred: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query was provided."),
        };

        let file_path = match args.next() {
            Some(q) => q,
            None => return Err("No file path was provided."),
        };

        Ok(Config {
            query: query,
            file_path: file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
