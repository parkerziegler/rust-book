fn main() {
    println!(
        "Value in cents of the penny: {}",
        value_in_cents(Coin::Penny)
    );

    println!(
        "Value in cents of the quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    println!("Adding 1 to 4: {:?}", plus_one(Some(4)));
    println!("Testing None handling: {:?}", plus_one(None));

    // If let is another useful syntactic tool that allows you to match
    // just a single pattern. For example:
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("Three!");
    } else {
        println!("Not three.");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // The match keyword enables pattern matching in Rust.
    // A value will be run through each pattern, and the first one that matches
    // will have its associated code executed.
    match coin {
        // Each statment in a match is called an "arm".
        // An arm is composed of a pattern (left side of =>) and the code to execute
        // if the pattern matches.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from state: {:?}", state);
            25
        } // Note that match does also support the placeholder _ to denote "all other
          // possible cases" and prevent tediously specifying cases that should all be
          // handled in the same way.
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(num) => Some(num + 1),
        None => None,
    }
}
