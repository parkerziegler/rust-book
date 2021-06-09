use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Collect cmd line arguments using the args function from the std::env module.
    // Note that this will only work if the arguments do not contain invalid Unicode.
    // In that case, use the args_os function.
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // The eprintln! macro will print to the stderr stream rather than stdout.
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
