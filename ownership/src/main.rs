fn main() {
    // The string literal type.
    // String literals are immutable, dealt with on the stack, and must have fixed size at compile time.
    let s = "hello";

    // The String type.
    // Strings are allocated on the heap and can have mutable size (unknown at compile time).
    // Memory must be requested from the memory allocator at runtime and returned at scope end.
    // Rust automatically calls drop when a variable's scope ends to free the allocated memory.
    let mut t = String::from(s);
    t.push_str(", world!");

    println!("{}", t);

    // In the example below, a allocates a String value on the heap.
    // b is bound to a, but _does not_ allocate a new String value on the heap.
    // Instead, it copies the _pointer_, _length_, and _capacity_, but still references the same heap value.
    let a = String::from("Parkie-Doo");
    let b = a;

    // This is known as a _move_ operation. a is now no longer referenceable — it's been "moved"
    // to b, and b now "owns" the value on the heap. The line below will error:
    // println!("{}", a);
    // main.rs(18, 9): move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
    // main.rs(19, 13): value moved here
    // main.rs(23, 20): value borrowed here after move
    // This is imperative to erasing concerns of double memory freeing.

    // If you do need to fully replicate data on the heap, you can clone it.
    let c = b.clone();
    println!("b: {}, c: {}", b, c);

    // Data that is fixed in size at compile time is kept on the stack, and can be shallowly copied
    // without needing to clone. For example:
    let d = 6;
    let e = d;
    println!("{}, {}", d, e);

    // Copy-able data structures implement the Cppy trait. Any data structure that implements the
    // Drop trait cannot also implement the Copy trait.

    // Function calls take ownership over a variable. For example, takes_ownership below takes overship
    // over b (b has been moved to takes_ownership). Attempting to reference b later would fail.
    takes_ownership(b);
    // makes_copy actually copies the value of e, so e can be referenced after this function call.
    makes_copy(e);

    // Return values from a function can give ownership of the returned value to the calling scope.
    let grad_school = gives_ownership();
    println!("Parker is going to graduate school at: {}", grad_school);

    let graduate_school = takes_and_gives_back(grad_school);
    println!(
        "Yep, Parker is really going to graduate school at: {}",
        graduate_school
    );
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i)
}

fn gives_ownership() -> String {
    let school = String::from("Berkeley");

    school
}

fn takes_and_gives_back(s: String) -> String {
    s
}
