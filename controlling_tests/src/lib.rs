pub fn prints_and_returns_10(value: i32) -> i32 {
    // By default, Rust captures anything printed to stdout if a test passes.
    // If a test fails, stdout will contain any printed output. You can change
    // this behavior by using cargo test -- --show-output
    println!("I got the value {}", value);
    10
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

// By default, tests in Rust run in parallel. If you want to run them sequentially,
// you can enforce that all tests run on a single thread using cargo test -- --test-threads=1.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes() {
        assert_eq!(10, prints_and_returns_10(8));
    }

    // We can use the ignore test to prevent a test from running by default with cargo test.
    // If we want to run all ignored tests, run: cargo test -- --ignored.
    #[test]
    #[ignore]
    fn it_fails() {
        assert_eq!(8, prints_and_returns_10(8));
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    // You can run a single test at a time by passing its name to cargo test. For example,
    // to run the code below we can run: cargo test add_two_and_three.
    #[test]
    fn add_two_and_three() {
        assert_eq!(5, add_two(3));
    }

    // You can also specify a more general string to cargo test and it will run all tests
    // matching that string. For example, running: cargo test add will run this test and the
    // two above it, all of which have add in their names. You can also use a test modules name
    // to run all tests in that module.
    #[test]
    fn add_two_and_one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
