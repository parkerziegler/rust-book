use test_organization;

// Here we bring the common module into scope.
mod common;

#[test]
fn it_adds_two() {
    // And now we call the setup function from the common module.
    common::setup();

    // Here we test the public API of the test_organization library.
    // We can run all integration tests in this module using the --test flag
    // with Cargo, i.e.: cargo test --test integration_tests
    assert_eq!(4, test_organization::adder(2, 2));
}
