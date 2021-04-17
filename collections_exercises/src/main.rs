use rand::Rng;
use std::collections::HashMap;

fn main() {
    get_summary_stats();

    println!("{}", pig_latinify(String::from("apple")));
    println!("{}", pig_latinify(String::from("first")));
}

fn get_summary_stats() {
    let mut random_integers = vec![];

    for _ in 0..10 {
        let random_int = rand::thread_rng().gen_range(1, 11);
        random_integers.push(random_int);
    }

    println!("Integers: {:?}", random_integers);

    let avg: i32 = random_integers.iter().sum::<i32>() / 10;
    println!("Average: {}", avg);

    random_integers.sort();
    let median = (random_integers[4] + random_integers[5]) / 2;
    println!("Median: {}", median);

    let mut random_integer_frequency = HashMap::new();

    for number in random_integers {
        let count = random_integer_frequency.entry(number).or_insert(0);
        *count += 1;
    }

    let mode = random_integer_frequency
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);
    match mode {
        Some(m) => println!("Mode: {}", m),
        None => println!("No mode found."),
    }
}

fn pig_latinify(s: String) -> String {
    let first_char = s.chars().nth(0);

    match first_char {
        Some(ch) => match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", s),
            first => format!("{}-{}ay", &s[1..], first),
        },
        None => String::from(""),
    }
}
