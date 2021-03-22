fn main() {
    // You can create a struct like so:
    let user1 = User {
        username: String::from("parkie-doo"),
        email: String::from("noreply@parkie-doo.sh"),
        sign_in_count: 0,
        active: false,
    };

    println!("{}", user1.email);

    // You can also create a mutable struct!
    let mut user2 = User {
        username: String::from("parkie-doo"),
        email: String::from("noreply@parkie-doo.sh"),
        sign_in_count: 0,
        active: false,
    };

    println!("{}", user2.email);

    // And mutate an individual property on the struct.
    user2.email = String::from("somereply@parkie-doo.sh");

    println!("{}", user2.email);

    let user3 = build_user(
        String::from("besshepner"),
        String::from("snailguitar@parkie-doo.sh"),
    );

    println!("{}, {}", user3.email, user3.username);

    // You can also spread values from a struct instance into another struct instance.
    // This is called struct update syntax.
    let user4 = User {
        email: String::from("darby@parkie-doo.sh"),
        username: String::from("darbette"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let green_channel = black.1;
    println!("The green channel of black is: {}", green_channel);
}

// A struct is a logical grouping of related values.
// It's similar to a tuple, but individual value access isn't order-based.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Functions can return structs.
fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: false,
    }
}

// You can also create a tuple struct, which is tuple-like but conveys
// a semantic meaning.
struct Color(i32, i32, i32);

// Unit structs can also be created — these are just empty structs, useful
// when you need to implement a trait but have no meaningful data to store.
struct Empty {}
