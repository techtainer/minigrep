use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);
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
    fn new(arg: &[String]) -> Config {
        let query = arg[1].clone();
        let filename = arg[2].clone();

        Config { query, filename }
    }
}
