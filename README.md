# Rust Book

This repository is just a fun place for me to track my learning of the Rust programming language via [the Rust Book](https://doc.rust-lang.org/book/). All the code examples you find here are taken directly from the Book, with some added comments to help me track new concepts.

## Running the Examples

If you want to run the examples in _this_ repository, you'll first need to follow [the Installation steps](https://doc.rust-lang.org/book/ch01-01-installation.html) for `rustup` to get a local working copy of the Rust compiler.

For the `hello_world` example you'll need to run `rustc` directly to compile the code, i.e.:

```sh
rustc main.rs
./main
```

All other projects use Cargo, Rust's package manager, to handle the build steps. You can compile and execute the examples in a single step using:

```sh
cargo run
```
