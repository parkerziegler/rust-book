// Below, we use the Rc smart pointer to maintain a count of
// references to a List. With an Rc type, multiple variables
// can reference the same data without taking ownership or
// needing to specify lifetime parameters. The referenced data
// won't be cleaned up until no more references point to it.
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a)); // 1

    // Use Rc::clone to increment the reference count to a.
    // Note that this does not make a deep copy like calling a.clome() would.
    let _b = Cons(4, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a)); // 2

    // Create c in a different scope so we can increment the reference count
    // to a and then see the count decrement when c is dropped at the end of
    // its scope.
    {
        let c = Cons(3, Rc::clone(&a));
        println!("{:#?}", c);
        println!("count after creating c: {}", Rc::strong_count(&a)); // 3
    }

    println!("count after c goes out of scope: {}", Rc::strong_count(&a)); // 2
}
