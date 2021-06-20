use add_one;

// We can run the binary crate below, in the context of our Cargo workspace,
// by running cargo run -p <crate_name>.
fn main() {
    let num = 10;
    println!(
        "Hello world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
