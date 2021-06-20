// To add documentation that describes the purpose of a crate itself, use
// the //! style of comment.

//! # Publishing Crates
//! `publishing_crates` is a set of utilities to make performing certain
//! calculations more convenient.

// In addition to standard comments, Rust has documentation comments.
// These use /// in lieu of // and are used to document the public API
// of a crate. To open the output HTML generated in target/doc, run
// cargo doc --open.

// Documentation comments should also conver situations in which the API:
// a) Panics
// b) Returns a Result<T, Err> type, including descriptions of the possible
// errors and when each may occur
// c) Unsafe code, including descriptions of invariants enforced by the
// underlying implementation.

// Code written in documentation comments get run as documentation tests!
// This is tremendously helpful in ensuring code documentation is up to date
// with the implementation itself.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing_crates::add_one(arg);
///
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
