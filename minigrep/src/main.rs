use std::env;

fn main() {
    // Collect cmd line arguments using the args function from the std::env module.
    // Note that this will only work if the arguments do not contain invalid Unicode.
    // In that case, use the args_os function.
    let args: Vec<String> = env::args().collect();

    // The program's name takes up index 0, so begin accessing args at index 1.
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
