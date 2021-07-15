// Declarative macros in Rust allow you to write something similar
// to a match expression. The key difference is that the input to a
// declarative macro is actually Rust source code that then gets ex-
// panded at compile time.
//
// The macro below is a simplified example of how the vec! declarative
// macro is implemented.

// Indicate the macro should be made available when the crate is brought
// into scope.
#[macro_export]
// Define the name of the macro, in this case vec.
macro_rules! vec {
    // Define $x to be the Rust expression encountered when using the macro.
    // The comma indicates that there may be a literal comma after the matched
    // expression. The * signifies that the pattern matches 0 or more of
    // whatever precedes the *.
    //
    // In vec![1, 2, 3], ( $x: expr ) matches three times, for 1, 2, and 3.
    ( $( $x: expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // The $()* here indicates that we should "fill in" this code whenever
            // the pattern matches with the concrete expression $x.
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
fn main() {
    println!("Hello, world!");
}
