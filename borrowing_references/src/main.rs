fn main() {
    let s1 = String::from("hello");

    // Here we pass a _reference_ into calculate_length.
    // calculate_length operates on the _reference_ to s1 rather
    // then taking ownership of the value.
    let length = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, length);

    // A mutable String passed as a mutable reference to change.
    // You can only have one mutable reference per let binding per scope.
    // You also cannot borrow a mutable reference if a previous immutable
    // reference has been borrowed.
    let mut s2 = String::from("hello");
    change(&mut s2);

    let r1 = &s2;
    let r2 = &s2;
    println!("r1: {} and r2: {}", r1, r2);

    // By this point, the immutable &s2 reference has been used for the last time,
    // thus it does not prevet a new mutable &s2 reference from occurring.
    let r3 = &mut s2;
    println!("r3: {}", r3);
}

// Passing a reference as a function parameter is called _borrowing_.
// References are _immutable by default_.
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope here.
  // However, because it is a reference and not a moved value, nothing happens!

// You can pass _mutable references_ as function parameters as well.
fn change(s: &mut String) {
    s.push_str(", world");
}
