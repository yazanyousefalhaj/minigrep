use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Self { query: args[1].clone(), filename: args[2].clone() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File: {}", contents);

    Ok(())
}