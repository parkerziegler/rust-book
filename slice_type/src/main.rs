fn main() {
    let str = String::from("Hello world");

    let result = first_word_naive(&str);
    println!("{}", result);

    // Rather than using a function to get a particular "slice" of a string,
    // we can use the slicing syntax.
    let hello = &str[0..5];
    let world = &str[6..11];
    println!("{} {}", hello, world);

    // If you start the slice at the beginning of the string you can omit the 0.
    let hello_2 = &str[..5];
    // Similarly, if you need the end of the string you can omit the last index.
    let world_2 = &str[6..];
    println!("{} {}", hello_2, world_2);

    // Specifying no indices takes the entire slice!
    println!("{}", &str[..]);

    let phrase = String::from("Gimme the first word!");
    println!("{}", first_word(&phrase[..]));
    println!("{}", second_word(&phrase[..]));

    // You can also use slices on arrays!
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", &arr[1..3]);
}

fn first_word_naive(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.trim().as_bytes();

    let mut first_space_index = 0;
    let mut second_space_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_space_index == 0 {
            first_space_index = i;
        } else if item == b' ' && second_space_index == 0 {
            second_space_index = i;
        } else if first_space_index != 0 && second_space_index != 0 {
            return &s[first_space_index + 1..second_space_index];
        }
    }

    &s[..]
}
