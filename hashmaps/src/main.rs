// Note that HashMap must be explicitly used.
// It is not included in the prelude of the standard library.
use std::collections::HashMap;

fn main() {
    // To create a hash map in Rust, use HashMap::new.
    // You can use generics to specify the key-value types,
    // but Rust can also infer them from the inserts below.
    let mut scores = HashMap::new();

    // We can now insert k-v pairs into a hash map using insert.
    scores.insert(String::from("Red Sox"), 10);
    scores.insert(String::from("White Sox"), 7);

    // You can also create a HashMap from a set of vectors.
    // The example below zips to vectors together and then calls .collect.
    let teams = vec![String::from("Red Sox"), String::from("White Sox")];
    let initial_scores = vec![10, 7];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash map keys or values will be copied for data structures that implement the Copy trait.
    // For data structures like Strings, the hash map will take ownership.
    let team_key = String::from("Red Sox");
    let team_value = 10;

    scores.insert(team_key, team_value);
    // The first line below errors, because the scores hash map now owns team_key (team_key has been moved).
    // println!("{}", team_key);
    // The line below is fine, because i32 implements the Copy trait.
    println!("{}", team_value);

    // To access a value out of a hash map in Rust, use .get.
    // .get returns an Option type, because the key access may not be valid.
    let white_sox_score = scores.get(&String::from("White Sox"));
    match white_sox_score {
        Some(s) => println!("The White Sox have {} runs.", s),
        None => println!("The White Sox are not playing."),
    }

    // To iterate over k-v pairs in a hash map, you for...in, just like for vectors.
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // To overwrite the value for a particular key in the hash map, just call .insert.
    scores.insert(String::from("Red Sox"), 9);
    println!("{:?}", scores);

    // To insert a k-v pair if and only if a key does not exist in the hash map, use .entry and .or_insert.
    scores.entry(String::from("Red Sox")).or_insert(12);
    scores.entry(String::from("Rays")).or_insert(3);
    println!("{:?}", scores);

    // We can also update a hash map based on an old value.
    // For example, we can count the frequency of characters in a sentence.
    let sentence = "hello world wonderful world";

    let mut frequency: HashMap<char, i32> = HashMap::new();

    for char in sentence.chars() {
        // or_insert returns a mutable reference to the value for the associated key.
        // We then dereference count in order to increment it.
        let count = frequency.entry(char).or_insert(0);
        *count += 1;
    }

    println!("{:?}", frequency);
}
