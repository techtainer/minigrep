use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(arg: &[String]) -> Result<Config, &str> {
        if arg.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}
