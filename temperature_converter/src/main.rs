use std::io;

fn main() {
    loop {
        println!("Choose a input temperature type: Celsius (c) or Fahrenheit (f).");
        let mut type_ = String::new();
        io::stdin()
            .read_line(&mut type_)
            .expect("Failed to readline");

        match &type_.trim()[..] {
            "c" => loop {
                println!("Please enter a Celsius value to convert to Fahrenheit.");

                let mut celsius = String::new();
                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to readline");

                println!(
                    "{} degrees Celsius is {} degrees Fahrenheit",
                    &celsius.trim()[..],
                    celsius_to_fahrenheit(celsius.trim().parse::<f64>().unwrap()),
                );
                break;
            },
            "f" => loop {
                println!("Please enter a Fahrenheit value to convert to Celsius.");

                let mut fahrenheit = String::new();
                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to readline");

                println!(
                    "{} degrees Fahrenheit is {} degrees Celsius",
                    &fahrenheit.trim()[..],
                    fahrenheit_to_celsius(fahrenheit.trim().parse::<f64>().unwrap())
                );
                break;
            },
            _ => println!("The input must be 'c' or 'f'."),
        }
        break;
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9. / 5. + 32.
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}
