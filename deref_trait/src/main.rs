use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    // Since we define y as a reference to x, we must dereference
    // it later to compare it to the i32 value 5.
    assert_eq!(*y, 5);

    // The dereference operator also works on Box. Here, it indicates
    // to Rust to follow the pointer to the value stored on the heap.
    let a = 5;
    let b = Box::new(a);

    assert_eq!(a, 5);
    assert_eq!(*b, 5);

    let alpha = 5;
    let beta = MyBox::new(alpha);

    assert_eq!(alpha, 5);
    // Because we implement the Deref trait on MyBox below, we can now
    // successfully use the deref operator. Under the hood, Rust translates
    // this to *(beta.deref()), allowing us to use the deref operator re-
    // gardless of whether the item being dereferenced is a regular reference
    // or implments the Deref trait.
    assert_eq!(*beta, 5);

    // Deref coercion allows for types that implement the Deref trait to
    // fulfill function signatures that would require explicit deferencing otherwise.
    // Below, name has the type MyBox<String>, and we pass a reference, &MyBox<String>,
    // to the hello function, which accepts an &str. Deref coercion uses the fact that
    // MyBox implements the Deref trait to coerce &MyBox<String> to &String, and the
    // standard library uses the built in deferencing of &String to &str to fulfill
    // the function signature.
    let name = MyBox::new(String::from("Parkie-Doo"));
    hello(&name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

// To implement the Deref trait for MyBox, we need to define implementations
// for each of its required methods.
impl<T> Deref for MyBox<T> {
    // Define the associated type.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(s: &str) {
    println!("Hello, {}!", s);
}
