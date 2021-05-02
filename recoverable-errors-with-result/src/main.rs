use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // The open function on File is an example of a function that returns the Result<T. E> enum type.
    // The Result type is useful for expressing a case where we may encounter a recoverable error.
    // For example, if we fail to open hello.txt below, we may want to recover from that error and create it instead.
    let f = File::open("hello.txt");

    // The Result enum has two members — Ok and Err.
    let _f = match f {
        Ok(file) => file,
        // Note that error here is the std::io::Error type.
        // It has a method, .kind, that returns a std::io::ErrorKind that we can match on.
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    example_using_unwrap();
    example_using_expect();
}

fn example_using_unwrap() -> () {
    // The unwrap method on the Result type will return the value inside the Ok variant if ok,
    // otherwise it will call the panic! macro for us.
    let _f = File::open("hello.txt").unwrap();
}

fn example_using_expect() -> () {
    // The expectt method allows us to define a specific panic! message if unwrapping fails.
    let _f = File::open("parkie-doo.txt").expect("Failed to open parkie-doo.txt");
}

// Sometimes you want to leave error handling up to calling code.
// This is what is called "propagating" the error.
// The caller of read_username_from_file is now responsible for handling the Ok and Error cases returned by this function.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // Return the Err member of the enum early if the file does not exist or cannot be read.
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    // read_to_string will read the contents of the file and store them in s.
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

// Rust has the ? operator available to help with the Result type.
// ? will return the inner value of Ok if no error occurred, or will return Err as if we had used the return keyword.
fn read_username_from_file_optimized() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // Note: the standard library does have an even shorter API for this.
    // fs::read_to_string("hello.txt")
}
