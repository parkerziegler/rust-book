fn main() {
    // The loop keyword instructs Rust to execute the block over and over, without stopping.
    // Uncomment this block to see this in action.
    // loop {
    //     println!("Hello, world!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // The break keyword allows you to break out of a loop.
            // You can return an expression after break and it will be
            // supplied as the return value to the caller.
            break counter * 2;
        }
    };

    println!("The value is: {}", result);

    // While loops are also supported in Rust.
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!");

    // As are for loops.
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    // For loops are the most common loop in Rust, and even constructs
    // that might be performed with a while loop are often done instead
    // with for loops.
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");
}
