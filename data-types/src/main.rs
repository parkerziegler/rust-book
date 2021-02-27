fn main() {
    // Mathematical operations.
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!(
        "Sum is: {}. Difference is: {}. Product is: {}. Quotient is: {}. Remainder is: {}",
        sum, difference, product, quotient, remainder
    );

    // Tuples.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);

    let z = tup.2;

    println!("The value of z is: {}", z);

    // Arrays. Note that arrays have a _fixed length_ in Rust. Vectors are allowed to grow
    // or shrink in size, so opt for a vector if you need a collection of variable size.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    // A shorthand for new Array(n).fill(var).
    let b = [3; 5]; // Returns [3, 3, 3, 3, 3].

    println!("The value of b is: {:?}", b);
}
