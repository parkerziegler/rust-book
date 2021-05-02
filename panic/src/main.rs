fn main() {
    // The panic! macro can be used when we want to throw an unrecoverable error in Rust.
    // For example, we could panic like so (uncomment the line below):
    // panic!("Crash and burn!");

    // Of course, you may encounter a panic by executing behavior that results in a panic
    // from another piece of code, whether that's the standard library or a crate.
    // For example, out-of-bounds access on a vector.
    let v = vec![1, 2, 3];

    v[99];

    // To debug the above call, you can run cargo run with RUST_BACKTRACE=1 to get a backtrace
    // of exactly what happened to cause the panic.
    //
    // Rust fully unwinds the stack when a panic is encountered, which involves walking back up
    // the stack and cleaning up data from each function that is encountered. To change this behavior,
    // you can abort without unwinding the stack. in Cargo.toml:
    // [profile.release]
    // panic = 'abort'
}
