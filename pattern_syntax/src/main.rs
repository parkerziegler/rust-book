struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum ColorSpace {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum ComplexMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(ColorSpace),
}

enum User {
    Name { id: i32 },
}

fn main() {
    // In Rust, you can match patterns against literals directly. For example;
    let x = 1;

    match x {
        1 => println!("x is 1!"),
        2 => println!("x is 2! Dope!"),
        3 => println!("x is 3! Wahoo!"),
        _ => println!("x is not 1, 2, or 3 :scream:"),
    }

    // You can also match named variables. Be careful with variable shadowing here;
    // match starts a new scope in which variables defined inside the match scope
    // will shadow those outside the scope.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("x is 50"),
        // x matches this pattern and the y variable above gets shadowed by the matched
        // value of y, 5.
        Some(y) => println!("Matched, y = {:?}", y), // 5
        _ => println!("Default case, x = {:?}", x),
    }

    // x and y in the outer scope are not shadowed here.
    println!("at the end: x = {:?}, y = {:?}", x, y); // Some(5), 10

    // You can also match multiple values in a single match arm, like so:
    let a = 2;

    match a {
        1 | 2 => println!("1 or 2"), // The 2 in this arm gets matched.
        3 => println!("3"),
        _ => println!("anything"),
    }

    // You can match inclusive ranges of values using the ..= operator.
    match a {
        1..=5 => println!("one through five"), // This arm gets matched since 2 is in the range {1..5}.
        _ => println!("anything!"),
    }

    // Ranges are also valid with the char type, but that's it — only numbers
    // and chars are allowed.
    let x = 'x';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // We can use patterns to destructure structs, even renaming fields into new
    // let bindings in the process.
    let p = Point { x: 20, y: 935 };

    let Point { x: a, y: b } = p;
    assert_eq!(20, a);
    assert_eq!(935, b);

    match p {
        Point { x: 0, y } => println!("On the y axis at y = {}!", y),
        Point { x, y: 0 } => println!("On the x axis at x = {}", x),
        Point { x, y } => println!("On neither axis. ({}, {})", x, y), // 20, 935
    }

    // We can also match enums and create new variables for their associated payloads.
    let msg = Message::ChangeColor(255, 0, 0);

    match msg {
        Message::Quit => println!("No data associated with Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
    }

    let complex_msg = ComplexMessage::ChangeColor(ColorSpace::Hsv(0, 160, 255));

    // Nested enums can also be matched! Below, we match on the ColorSpace enum
    // inside of the ComplexMessage enum — two levels deep!
    match complex_msg {
        ComplexMessage::ChangeColor(ColorSpace::Rgb(r, g, b)) => {
            println!("Change color to R: {}, G: {}, B: {}", r, g, b)
        }
        ComplexMessage::ChangeColor(ColorSpace::Hsv(h, s, v)) => {
            println!("Change color to H: {}, S: {}, V: {}", h, s, v)
        }
        _ => (),
    }

    // We can also do deep destructuring of tuples and structs at the same time.
    let ((feet, inches), Point { x, y }) = ((5, 10), Point { x: 20, y: 935 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    // Here, we don't care about the interior values in the matched option types,
    // just their presence. This is the perfect use case for _.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("setting is {:?}", setting_value);

    let numbers = (4, 16, 19, 8, 13);

    // We can also use the _ charactr to ignore only specific parts of a pattern.
    match numbers {
        (first, _, third, _, fifth) => {
            println!("first: {}, third: {}, fifth: {}", first, third, fifth)
        }
    }

    // We can still bind a variable but signal that it's unused by prefixing the
    // variable name with _. Note that, because this still creates a binding, it's
    // different from just _.
    let _unused_x = 20;

    let s = Some(String::from("s"));

    // Here, we move the value of s into _s. In doing so, we've transferred ownership
    // and now can no longer use s. L156 below, if uncommented, will error.
    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("s is: {:?}", s);

    // If we only want to match part of a value that can hold multiple parts, use the .. syntax.
    // ..  will expand to as many values as it needs to.
    let q = Point { x: 0, y: 0 };

    match q {
        Point { x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => println!("first is {}, last is: {}", first, last),
    }

    // match guards are extra conditionals that allow you to add conditional logic to patterns.
    let r = Some(4);

    match r {
        Some(n) if n < 5 => println!("n is less than 5: {}", n),
        Some(n) => println!("n is {}", n),
        None => (),
    }

    // match guards can also be used with variables from an outer scope.
    let d = 10;
    let e = Some(5);

    match e {
        Some(50) => println!("50"),
        Some(n) if n == d => println!("n = d = 10"),
        _ => println!("default case, e is {:?}", e),
    }

    // match guards can also be combined with the | operator.
    // These combinations are exploded, meaning each | pattern is combined
    // with the match guard.
    let f = false;

    match d {
        9 | 10 | 11 if f => println!("yes"), // Although d matches 10, f is false, so this arm doesn't match.
        _ => println!("no"),
    }

    let user = User::Name { id: 4 };

    // The @ operator allows us to create a variable at the same time that we're testing
    // that variable for a certain condition. This is useful for testing numbers in a range
    // that we later need to use in the match arm.
    match user {
        User::Name {
            id: id_variable @ 3..=7,
        } => println!("Found id in range: {}", id_variable),
        User::Name { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        User::Name { id } => println!("Found some other id: {}", id),
    }
}

// The _ character will match any value but not bind to it. It's a useful wildcard
// when we're not interested in using the ignored value.
fn foo(_: i32, y: i32) {
    println!("This function only uses y: {}", y);
}
