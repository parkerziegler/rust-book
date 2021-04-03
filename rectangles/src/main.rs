#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Note the immutable borrow of self.
    // It is quite rare for a method to take ownership over self.
    // This typically only happens if we need to transform the Struct
    // and ensure consumers cannot access the previous Struct instance.
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
    fn can_hold(&self, rect_inner: &Rectangle) -> bool {
        &self.width > &rect_inner.width && &self.height > &rect_inner.height
    }
    // square is an _associated function_.
    // It is a function that does not receive a Struct instance.
    // Often, it's used for constructors to create a new Struct from a set of arguments.
    fn square(dimension: u32) -> Rectangle {
        Rectangle {
            width: dimension,
            height: dimension,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is: {}", area(&rect));

    // This line will error.
    // Structs do not implement the Display trait by default, since there is
    // ambiguity in how they may be displayed.
    // println!("The rect: {}", rect);

    // The Debug format will also fail without additional derive annotations,
    // since Rectangle does not implement the Debug trait.
    // Adding #[derive(Debug)] on L1 above is what specifies that Rectangle should
    // derive the base Debug trait.
    println!("The rect is: {:?}", rect);

    // We can get more robust pretty printing (multi-line) using:#?
    println!("The rect is: {:#?}", rect);

    // Methods can implemented on Structs using impl.
    // For example, area can be a method implemented on the Rectangle type.
    println!("The area of the rectangle is: {}", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 60,
        height: 15,
    };

    println!("rect can hold rect3: {}", rect.can_hold(&rect3));

    let square = Rectangle::square(50);

    println!("The area of square is: {}", square.area());
}

// Use an immutable borrow to access the passed struct.
// This way, the caller can maintain ownership over the passed value.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
