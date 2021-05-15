fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Demonstrating how generic lifetimes affect calling the longest function.
    // Below, we can access result because we derive its value in the lifetime of
    // string4, which is smaller than string3.
    let string3 = String::from("abcd");
    {
        let string4 = "xyz";
        let result = longest(string3.as_str(), string4);
        println!("The longest string is {}", result);
    }

    // The following will not compile. We define result outside of the lifetime of
    // string6. However, our lifetime restrictions on longest state that string6 would
    // need to live as long as result, which it does not.
    let string5 = String::from("long string is long");
    let result;
    {
        let string6 = String::from("xyz");
        result = longest(string5.as_str(), string6.as_str());
    }
    println!("The longest string is {}", result);
}

// This function demonstrates an attempt to access a value outside of its lifetime.
// In Rust, values have lifetimes that denote when they can be referenced or accessed.
// In the example below, we borrow the value of x and assign it to r, and then attempt
// to print out r after x has gone out of scope. The error will indicate that the borrowed
// value x does not live long enough to make this access safe.
fn invalid_lifetime() {
    let r;

    {
        let x = 5;
        // x does not live long enough.
        // r is attempting to access memory that has been deallocated.
        r = &x;
    }

    println!("r: {}", r);
}

// The longest function uses lifetime generics.
// Lifetime generics indicate relationships between _how long_ references live.
// In the implementation of longest, we're saying that x and y have the same generic lifetime,
// 'a, and that the return value must have the same lifetime as these parameters.
// Concretely, these constraints mean that 'a is the generic lifetime for the _smaller_ of the
// real lifetimes of x and y. If x or y had a smaller scope, that would be the value of 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs can also use lifetime generics.
// The following constrait on part indicates that an instance of ImportantExcerpt
// cannot outlive the string slice reference it holds in part.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn get_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.' character.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Rust does have lifetime elision rules. These are predictable cases where lifetime annotations
// are added behind the scenes by the compiler. For example, the function below operates on references,
// but its definition contains no lifetime parameters.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// If Rust cannot infer the lifetimes of references based oon the lifetime elision rules, it will
// throw a compiler error which can be fixed by annotating the lifetimes explicitly.

// The lifetime elision rules for methods allow most method signatures to go unannotated.
impl<'a> ImportantExcerpt<'a> {
    // The first lifetime elision rule states that a function (or method) with one parameter has one
    // input lifetime, and that its output lifetime is the same is its input lifetime.
    fn level(&self) -> i32 {
        3
    }
    // The third lifetime elision rule states that a method referencing &self will have the same
    // output lifetime as &self, so annotation is not needed here either.
    fn announce_self(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Rust has the notion of the special 'static lifetime, indicating that a value lives for the
// duration of the program. String literals automatically have the 'static lifetime assigned.
fn static_lifetime() {
    let s: &'static str = "Hey Doo!";
}
