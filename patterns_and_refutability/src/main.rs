// Patterns come in two forms — refutable and irrefutable.
//
// Refutable patterns are distinguished by their ability _not_ to match.
// For example, in the pattern:
// if let Some(x) = a_value {
// the pattern will fail to match if a_value is None.
// if let and while let accept refutable and irrefutable patterns, although
// the compiler will warn on using irrefutable patterns in favor of using constructs
// designed for irrefutable patterns.
//
// Irrefutable patterns always match.
// For example, in the pattern:
// let x = 5;
// x will always be matched to the value 5.
// Function parameters, let statements, and for loops only accept irrefutable patterns.

fn main() {
    let some_other_value: Option<i32> = None;

    // Try uncommenting the line below to see the compiler error.
    // Rust will error here, indicating that let bindings require an irrefutable pattern.
    // Since the pattern Some(x) can be refuted by the value None (or rather, let Some(x)
    // doesn't cover every pattern that Option<i32> could result in), Rust errors.
    // let Some(x) = some_other_value;

    // if let is a more appropriate construct to use with a refutable pattrn like the one
    // above. The block will only be run if some_other_value matches the pattern Some(x).
    if let Some(x) = some_other_value {
        println!("x is {}", x);
    };

    // Here we use an irrefutable pattern with an if if let statement — 5 is always matched
    // to the pattern x. The compiler will warn us with the irrefutable_let_patterns warning.
    if let x = 5 {
        println!("x is {}", x);
    };

    // match arms must use refutable patterns, except for the last arm, which uses a catch all
    // if we don't want to exhaustively enumerate values.
    let a = 1;

    match a % 2 == 0 {
        true => println!("a is even!"),
        false => println!("a is odd!"),
    }
}
