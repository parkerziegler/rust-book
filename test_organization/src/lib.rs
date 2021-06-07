fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn adder(a: i32, b: i32) -> i32 {
    a + b
}

// The cfg attribute in Rust is shorthand for configuration and indicates to
// Rust that what follows should only be included given a certain config option.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // Rust can also test private functions, unlike other languages.
    #[test]
    fn it_adds_integers() {
        assert_eq!(10, internal_adder(3, 7))
    }
}
