use std::net::IpAddr;

// In general, it's wise to return a Result type and let consuming code handle your errors
// rather than panic!-ing. However, there are certain cases where allowing panic! is helpful.
// These include:
//   - Example code that uses unwrap or expect.
//   - Test code, in which panic! is used to mark failures.
//   - Early prototypes, where you can use unwrap or expect to signal where more robust error handling is needed.
fn main() {
    // Another case where we don't need to panic is in cases where we can logically
    // rule out the Err case, but the compiler can't.
    // For example, the below hardcoded string will always be a valid IP, so we'll never encounter an Err.
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // panic!-ing is also advisable in cases where your code could end up in a bad state.
    // This occurs when an invariant is violated or an API contract broken.
    // For example, if someone passes in bad values to your code, panic!-ing is important.
    // If the potential for failure is _expected_, such as a rate limiting error on an API,
    // then a Result type is more appropriate.

    // If your code performs checking on particular values, you should panic! if the validity
    // of those values cannot be safely verified. This is important to avoiding security
    // vulnerabilities and to uphold a function's API contract.

    // Another use case for panic! is to throw in the context of dependent type failures.
    // For example, we can create a struct that validates input on creation such that passing
    // certain values would result in a panic!

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("value must be between 1 and 100.");
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
