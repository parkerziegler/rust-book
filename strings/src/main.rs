fn main() {
    // Strings in Rust come in two forms — the string slice (str and &str) and the String type.
    //
    // str, or string slices, refer to some UTF-8 encoded string data, typically referenced elsewhere using a borrow.
    // String literals are str types stored in a program's binrary.
    //
    // String are growable, mutable, owned UTF-8 encoded string types.

    // To create a new empty String, use String::new().
    let mut s = String::new();

    // To create a new String from a string literal, use either to_string() or String::from().
    let library_name = "FormidableLabs/renature";
    let lib = library_name.to_string();
    let lib = String::from(library_name);

    // Strings can grow and size and have their contents change, similar to vectors.
    s.push_str("FormidableLabs/rescript-urql");
    println!("{}", s);

    // Note that psuh_str above takes a string slice, &str, as its only parameter.
    // This ensures that push_str doesn't take ownership of the passed in str.

    // You can also push a single character using .push().
    let mut t = String::from("lo");
    t.push('l');
    println!("{}", t);

    let s1 = String::from("Formidable");
    let s2 = String::from("Labs");
    // To concatenate Strings, you can use the + operator.
    // The simplified signature of String concatenation is (self, s: &str) -> String.
    // Rust coerces (uses a deref corecion) the second argument &s2 from &String to &str.
    let s3 = s1 + &s2;
    println!("{}", s3);

    // For more complex string concatenation we use the format! macro.
    // Format does not print the String, it just returns the formatted string and _does not_ take ownership of the passed in Strings.
    let s4 = String::from("/");
    let s5 = String::from("renature");
    let lib = format!("{}{}{}", s3, s4, s5);

    // Characters in Rust cannot be accessed by index.
    // Internally, the String type is a wrapper over Vec<u8>, which is UTF-8 encoded.
    // In this way, Strings are really stored as collections of bytes.
    // Because not all characters are represented using the same number of bytes, we cannot reliably access them by indices.

    // From Rust's perspective, there are three relevant ways of looking at strings: as bytes, scalars, and graphene clusters.
    // You _can_ index a string slice, but it will result in a panic if the slice results in a byte access that is invalid.
    let hello = String::from("hello");
    println!("{}", &hello[0..4]);

    // The above prints "hell", but the following would panic because the first byte is not a byte boundary.
    // let hello = "Здравствуйте";
    // println!("{}", &hello[0..1]);

    // To safely iterate over the characters of a string, use .chars().
    for c in hello.chars() {
        println!("{}", c);
    }

    // You can also iterate over the bytes of a string, using .bytes().
    for byte in hello.bytes() {
        println!("{}", byte);
    }
}
