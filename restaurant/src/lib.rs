// The mod keyword defines a module in Rust.
mod front_of_house {
    // Modules can be nested inside one another.
    // They serve to group related functionality in a single semantic scope.
    // By default, functions, structs, etc. are private by default.
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Items (functions, structs, enums, etc.) in a child module can see their parent
    // module's items, but the opposite is not true. You can explicitly make items in
    // a child module public using the pub keyword.
    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Use the pub keyword to make a crate data structure as publicly accessible by crate consumers.
pub fn eat_at_restaurant() {
    // Absolute path reference to library crate.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path reference to the front_of_house module.
    front_of_house::hosting::add_to_waitlist();
}

fn serve_custom_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        // To access functions (and other items) in an upper scope, use the super keyword
        // in the relative path.
        super::serve_custom_order();

        // You can also use the absolute path.
        crate::serve_custom_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Guava"),
            }
        }
    }

    // By default, all members of an enum are public if the enum itself is public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_fancy_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // We can mutate the public field of Breakfast struct.
    meal.toast = String::from("Potato Rosemary");

    // But we cannot mutate the private field seasonal_fruit.
    // meal.seasonal_fruit = String::from("Strawberry");
}

// The use keyword allows us to bring a full module path into scope.
use crate::front_of_house::hosting;

// You can also use relative paths to bring the module into scope with self.
// Secondly, you can prefix a use statement with pub to indicate that consuming
// code can also use the module. This is called re-exporting.
pub use self::front_of_house::serving;

// It's idiomatic in Rust to reference functions using their parent module path.
// This helps make it clear where the accessed function actually lives.
// Conversely, referencing structs, enums, etc. happens up to the value itself.
use std::collections::HashMap;

// The exception to this is when you're bringing in two values with the same name
// from separate modules. You can also post-fix with the as keyword to alias the use call.
use std::fmt;
use std::io;

pub fn eat() {
    // We can now calls functions from the front_of_house::hosting module
    // without needing the relative or absolute path reference.
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    fn fn_1() -> fmt::Result {
        Ok(())
    }

    fn fn_2() -> io::Result<()> {
        Ok(())
    }
}

// To bring in items that share a path, we can do the following:
use std::{alloc, cmp::Ordering};

// To bring in an item and one of its children, use self in the use statement, for example:
// use std::io::{self, Write};

// To bring in all public items in a path, use the wildcard glob (*)
use std::collections::*;

// You can also split code across multiple files and bring it into scope.
// The statement below instructs Rust to load the contents of a file with the module name.
mod part_of_house;

pub use crate::part_of_house::part_hosting;

fn eat_at_part_of_house() {
    part_hosting::add_to_waitlist();
}
