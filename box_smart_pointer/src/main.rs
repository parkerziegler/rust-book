use crate::List::{Cons, Nil};

fn main() {
    // A Box is a smart pointer that allows us to store data on the heap
    // while keeping just the pointer metadata on the stack. Deallocation
    // will happen when the box goes out of scope, which will include removing
    // the data from the heap in addition to the pointer on the stack.
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Cons list: {:#?}", list);
}

// Boxes are useful for implementing recursive types like the one below.
// Rust must know the size of data it is storing at compile time, which
// will be Infinity for a recursive type. To amend this, we use Box to
// signal to Rust that we'll store a pointer on the stack that points to
// data on the heap. This patten is sometimes called "indirection".
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
