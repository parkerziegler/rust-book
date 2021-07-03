fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // Patterns in Rust take many different shapes. We can combine if let with
    // else if and else if let to do more sophisticated matching with unrelated
    // patterns, which we can't do with match.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is teal day!");
        // Below we introduce a new shadowed age variable that contains the value
        // inside of the Ok variant.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let allows us to run a while loop as long as a pattern continues to match.
    let mut stack = vec![];

    for i in 1..4 {
        stack.push(i);
    }

    while let Some(top) = stack.pop() {
        println!("top {}", top);
    }

    // for loops also have patterns in them. In the structure for x in y, x is the pattern.
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Even let stataments are patterns of the form let PATTERN = EXPRESSION.
    // In a statement like let x = 5, Rust is internally constructing a pattern
    // saying, "bind what matches here to the variable x". Destructuring is a
    // clearer form of pattern matching.
    let (_x, _y, _z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);
}

// Function parameters are another good example of a pattern, as shown by the
// destructuring of this tuple below.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
