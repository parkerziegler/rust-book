use std::env;
use std::process;

use minigrep_2::Config;

fn main() {
    // Pass the iterator returned by env::agrs() to Config::new.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // The eprintln! macro will print to the stderr stream rather than stdout.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_2::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
