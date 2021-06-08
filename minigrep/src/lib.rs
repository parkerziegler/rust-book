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
    // The ? operator ensures we return a Result type from read_to_string
    // but leave it up to the caller to handle the error.
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Indicate that we're calling run for its side effects only by returning
    // the Ok enum member wrapping the unit type.
    Ok(())
}

// We use a lifetime parameter to indicate that the returned vector from search
// lives as long as the contents reference.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
