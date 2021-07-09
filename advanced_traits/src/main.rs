// Associated types are similar to abstract types in a language like OCaml.
// They're often used with in Trait definitions, and they act as a stand-in
// type to represent the concrete type used by another type implementing the trait.
//
// An example is the definition of the Iterator trait.
pub trait Iterator {
    // Here we have the associated type, Item.
    // Item represents the type of thing a type that implements Iterator
    // is actually iterating over (i.e. an i32, a String, etc.).
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Using associated types has some advantages over generics. For example,
// we could define Iterator like so:
pub trait IteratorAlt<T> {
    fn next(&mut self) -> Option<T>;
}

// The downside is that we can now have multiple implementations of Iterator
// with different concrete types to stand in for the generic (i.e. i32, String).
// Everywhere we use the next method, we'd need to specify, or hope Rust can infer,
// the proper concrete type for the generic T.
//
// With associated types, we can only have one implementation operating on one concrete type.
// We give up some flexibility to restrict how many times we can implement the trait on a
// particular struct.

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Rust does allow for some degree of operator overloading through traits.
// For example, we can implement the Add trait for the Point struct, so when
// we use the + operator on two Points, the add fn gets run.
impl Add for Point {
    // The associated type, Output, defines the output type returned by add.
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// The default implementation for Add uses a placeholder generic type to represent
// the associated type, like so:
trait AddEx<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// Here, we show an example of implementing Add with different types.
// This implementation uses Meters to represent the Rhs (right-hand side)
// type for the generic T. This indicates we're defining how to add a
// Millimeters and a Meters struct together.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Nothing in Rust disallows us from implementing a trait on a struct
// that has a method with the same name as a trait method. For example,
// we implement the Pilot and Wizard traits on the Human struct below,
// all of which define a fly method.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog {}

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppers!")
    }
}

// Rust also allows us to define super traits. Below, we state that the
// Display trait is a supertrait of OutlinePrint. Any type implementing
// OutlinePrint must also implement Display.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // The self.to_string call is only possible because we've enforced
        // the Display trait as a super trait.
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// In order to implement OutlinePrint for Point, we must also implement
// its super trait, Display, which we do on L149 below.
impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Typically, we can only implement a trait on a type as long as either
// the trait or the type are local to our crate. One way to get around this
// restriction is to use the *newtype pattern*, borrowed from Haskell. This
// entails creating a Wrapper tuple struct around a type, which is now local.
//
// Below, we use the newtype pattern to implement the Display trait on the Vec
// type, which would normally be disallowed. The downside is that all of the
// other methods on the Vec type are not implemented on Wrapper; we'd have to
// reimplement them ourselves.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human {};
    // By default, Rust will call the method implemented on the struct
    // itself before looking at trait methods with the same name.
    person.fly(); // *waving arms furiously*

    // In order to call the fly method from Pilot or Wizard, we need
    // to use an alternate, more explicity syntax.
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up!

    // The line below is an example of using fully qualified syntax in Rust.
    // This syntax is useful for associated functions that don't take a self
    // parameter. Since both struct Dog and the Animal trait have a baby_name
    // associated function, we need some mechanism for helping Rust determine
    // which one to call.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppers!

    let origin = Point { x: 0, y: 0 };

    origin.outline_print();

    let w = Wrapper(vec![String::from("Parkie"), String::from("Doo")]);
    println!("w = {}", w);
}
