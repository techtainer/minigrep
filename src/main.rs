use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong");

    println!("With text:\n{}", contents);
}

fn parse_config(arg: &[String]) -> (&str, &str) {
    let query = &arg[1];
    let filename = &arg[2];

    (query, filename)
}
