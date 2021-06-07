use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        // The program's name takes up index 0, so begin accessing args at index 1.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// The Box<dyn Error> below is a trait object. It indicates that, in the
// error case, run will return some value that implements the Error trait
// rather than a specific type.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // The ? operator ensures we return a Result type from read_to_string
    // but leave it up to the caller to handle the error.
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // Indicate that we're calling run for its side effects only by returning
    // the Ok enum member wrapping the unit type.
    Ok(())
}
