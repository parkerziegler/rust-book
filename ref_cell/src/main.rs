#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Recall that we cannot mutably borrow an immutable reference.
    // For example, the code below will throw a compiler error.
    // cannot borrow `a` as mutable, as it is not declared as mutable
    // let a = 5;
    // let b = &mut a;
    //
    // The RefCell<T> smart pointer allows us to mutate a value internally
    // while signaling to consumers of that value that it is immutable.
    // This is called the interior mutability pattern.

    // We can combine Rc and RefCell together to allow for mutable data
    // with multiple owners!
    //
    // Start by defining an initial value that we will mutate using the
    // interior mutability pattern.
    let value = Rc::new(RefCell::new(5));

    // Create a small cons list with one value and Nil.
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // Create two cons lists that both own a via a reference count.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Mutate the initial value, 5, to 15. Here we are _mutating_ data in
    // the cons list that is owned by _both_ b and c via the reference count.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a); // Cons(15, Nil)
    println!("b after = {:?}", b); // Cons(3, Cons(15, Nil))
    println!("c after = {:?}", c); // Cons(4, Cons(15, Nil))
}
