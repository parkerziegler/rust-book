fn add_one(x: i32) -> i32 {
    x + 1
}

// The fn type annotated below is called a function pointer. This type
// allows us to pass functions as arguments to other functions. fn is
// different than the Fn trait used with closures — in fact, it implements
// the Fn, FnMut, and FnOnce traits, so it can be used wherever a closure
// can be used.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("Answer: {}", answer);

    // We can use either closures or functions in a place like mapping over
    // items in an iterator.
    let list_of_numbers_cl = vec![1, 2, 3, 4, 5];
    let list_of_strings_cl: Vec<String> =
        list_of_numbers_cl.iter().map(|n| n.to_string()).collect();

    let list_of_numbers_fn = vec![1, 2, 3, 4, 5];
    let list_of_strings_fn: Vec<String> =
        list_of_numbers_fn.iter().map(ToString::to_string).collect();

    // Tuple structs and tuple struct enums are implemented under the hood as
    // functions returning an instance that's constructed from their arguments.
    // Using this fact, we can use these initializers as function pointers, allowing
    // us to create new instances of an enum value.
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    for status in list_of_statuses {
        println!("Status: {:?}", status);
    }
}

// If you need to return a closure, return a trait object instead!
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
