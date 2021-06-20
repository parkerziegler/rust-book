pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Tests in a Cargo workspace are run at the top level using cargo test.
// To run tests for a single crate, use cargo test -p <crate_name>.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(2), 3);
    }
}
