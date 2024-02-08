use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error occurred while parsing arguments: {err}");
        process::exit(1);
    });

    println!("Seraching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            let message = format!(
                "2 arguments should be given but received {}",
                args.len() - 1
            );
            return Err(message);
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
