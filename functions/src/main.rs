fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(5);

    another_function_with_multiple_parameters(5, 6);

    let b = five();
    println!("The value of b is: {}", b);

    let c = plus_one(b);
    println!("The value of c is: {}", c);
}

// Function declaration order does not matter in Rust.
// Functions must be declared _somewhere_ in scope, but not necessarily declared before use.
fn another_function() {
    println!("Another function.");
}

// Function parameters _must_ be type annotated.
fn another_function_with_parameters(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_multiple_parameters(x: i32, y: i32) {
    println!("The value of x is: {} and the value of y is: {}", x, y);
}

// In Rust, _statements_ are differentiated from _expressions_.
// A statement is an instruction performing some action _but not_ returning a value.
// For example:
//   let x = 6;
// This is a _statement_. Another example of a statement includes a function declaration.
// An expression can be many things, including a literal value, the resutl of calling a function,
// or an entire block.
// For example:
//   {
//      let x = 5;
//      x + 1
//   }
// This is an expression evaluating to 6. Note the lack of semicolon after x + 1,
// indicating that this is also an _expression_ and not a statment.

// Functions with return values.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
