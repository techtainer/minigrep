use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(arg: &[String]) -> Result<Config, &str> {
        if arg.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();

        Ok(Config { query, filename })
    }
}
