// Rust allows for type aliasing, similar to TypeScript.
// Values using aliased types will be treated as if they
// had the underlying type.
type Kilometers = i32;

// Type aliasing is mostly useful for shortening long types
// and being able to refer to them in a more semantic way.
type Thunk = Box<dyn Fn() + Send + 'static>;

// Rust also has a never type, similar to TypeScript, denoted
// by the ! character. In PL circles, this is called the empty
// type and represents the situation when a function never returns.
// Functions that never return are called *diverging functions*.
//
// Formally, any expression of type ! can be coerced to any other type.
// match arms that yield continue, the unwrap method on Option<T>, and a
// loop without a break expression are all examples of the ! type in use.

// Rust also has the notion of dynamically sized types (DSTs). str (not &str)
// is one example of a DST, since we can't know how long a string is until
// runtime. This means we can't make a variable of str, nor take an argument
// of type str in a function.
//
// The golden rule of DSTs is that they must always be behind some kind of pointer,
// such as a reference, Box, Rc, etc.
//
// The Sized trait in Rust is useful for working with DSTs. The Sized trait
// determines whether or not a type's size is known at compile time. By default,
// generic functions will only work on types that have a known size at compile
// time. You can relax this restriction with the following syntax:
fn generic<T: ?Sized>(t: &T) {}

// The above is read as "T may or may not be sized." This syntax is unique to Sized.

fn main() {
    let x: i32 = 15;
    let y: Kilometers = 3;

    let _f: Thunk = Box::new(|| println!("Thunkify me cap'n!"));

    println!("x + y = {}", x + y);
}
