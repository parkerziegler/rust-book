fn main() {
    let number = 7;

    // Traditional if-else control flow.
    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    // If-else if-else control flow.
    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisble by 4, 3, or 2")
    }

    // You can assign the result of an if expression to a let binding.
    // Note that assigning the result of an if expression requires the
    // type of both arms, or branches, to be the same (i32 here).
    let x = if true { 3 } else { 4 };

    println!("The value of x is: {}", x);
}
