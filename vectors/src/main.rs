fn main() {
    // Vectors store an arbitrary number of values of the same type next to each other in memory.
    // Because they are variable in size, Rust stores vectors on the heap.
    let v: Vec<i32> = Vec::new();

    // Vectors are more commonly instantiated with the vec! macro and some initial values.
    // In this case, the type can be inferred from the initial values.
    let v = vec![1, 2, 3];

    // You can add values to vectors, but they must be mutable.
    let mut w = vec![1, 2, 3];
    w.push(4);

    // There are two ways to reference values in a vector.
    // You can access by index, like so:
    let x = &w[2];

    // Or you can pattern match on the get call.
    match w.get(1) {
        Some(first) => println!("The value at index 1 is: {}", first),
        None => println!("There is no value at index 1."),
    }

    // The  former approach is useful if you want to panic on out-of-bounds access.
    // The latter is useful when out-of-bounds access may be rare and you want to recover from it.

    // Recall that you cannot mix an immutable reference and a mutable reference in the same scope.
    // More specifically, we cannot push to a Vector and then immutably borrowing from it afterwrds.
    // Uncommenting L31 would throw an error, since x immutably borrows w on L18.
    w.push(5);
    // println!("The element at index 2 of w is: {}", x);

    // You can also iterate over both immutable and mutable vectors.
    let z = vec![100, 99, 98];
    for i in &z {
        println!("The value of i is: {}", i);
    }

    let mut a = vec![100, 99, 98];
    for i in &mut a {
        // Deference i using the * operator to allow for mutating it.
        *i += 50;

        println!("The value of i is: {}", i);
    }

    // If you need to store values of different types in a vector, use an enum data structure.
    // All values will be of the enum type, but  each enum member can hold different types within it.
    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Float(1.5237),
        SpreadsheetCell::Text(String::from("Rose Gold")),
    ];

    // Rust needs to know at compile time how much memory needs to be allocated on the heap,
    // hence the need to know the type of each stored element.
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
