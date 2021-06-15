use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // We pass the args iterator to new, and iterating over it will mutate
    // its internal state, hence the mut keyword. The string slice error type
    // uses the 'static lifetime since we're only ever returning string literals.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // The program name is the first arg, skip it.
        args.next();

        // Match on successive calls to args.next().
        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query specified."),
        };
        let filename = match args.next() {
            Some(f) => f,
            None => return Err("No file name specified."),
        };

        // Check if the end user's environment has set the CASE_INSENSITIVE env var.
        // If the env var is set, we'll get an Ok result with the value of the env var.
        // If not, we'll receive an Err.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// The Box<dyn Error> below is a trait object. It indicates that, in the
// error case, run will return some value that implements the Error trait
// rather than a specific type.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The ? operator ensures we return a Result type from read_to_string
    // but leave it up to the caller to handle the error.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // Indicate that we're calling run for its side effects only by returning
    // the Ok enum member wrapping the unit type.
    Ok(())
}

// We use a lifetime parameter to indicate that the returned vector from search
// lives as long as the contents reference.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
