use std::fmt::format;

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got: {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // The following line brings all code in the outer scope into the test
    // scope for the tests module.
    use super::*;
    // The #[test] attribute identifies this function as a test function.
    // This allows the test runner to identify the function as a test,
    // as opposed to a function used for setup in a test environment.
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn intentional_failure() {
        let list = vec![1, 2, 3];

        // We're intentionally accesing an index in the list vector beyond its bounds.
        assert_eq!(list[3], 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // The assert! macro is useful for testing boolean logic.
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 5,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // The assert_eq! macro compares two values.
        // It's especially useful because it will print
        // both values to stdout if the values are not equal.
        // Note that this requires the values being compared
        // to implement the PartialEq and Debug traits.
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Steve");
        assert!(
            result.contains("Steve"),
            "Greeting did not contain name, value was: {}",
            result
        )
    }

    // You can use the [should_panic] attribute after the [test]
    // attribute to indicate that a test should panic and only pass
    // if it does indeed panic. You can pass a substring to expected
    // to indicate the message you expect to be associated with the panic.
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // You can also use Result<T, E> in tests rather than the assert macro.
    // Note that you cannot use the [should_panic] attribute in a test that
    // uses Result<T, E> and should instead return the Err type.
    #[test]
    fn it_still_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4 😱"))
        }
    }
}
