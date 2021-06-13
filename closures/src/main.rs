use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures can use values from their environments, unlike functions.
    // Below, the closure equal_to_x stores x in memory and can reference
    // it when called later.
    let x = 4;
    let equal_to_x = |z| z == x;

    println!("4 equals x: {}", equal_to_x(4));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Today, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// The Cacher struct allows us to store a closure in this struct
// and only execute the closure when requested. We use the trait
// bound Fn to signal that the generic type T must implement the
// Fn trait to be a valid value for calculation.
struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + std::hash::Hash,
    V: Copy,
{
    calculation: T,
    value: HashMap<K, V>,
}

// Note that all methods below are private, ensuring that no code
// that instantiates a Cacher can modify or access internals.
impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + std::hash::Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn works_with_different_types() {
    let mut c = Cacher::new(|a: &str| a.len());

    let v1 = c.value("Hello");
    let v2 = c.value("Hi");

    assert_eq!(v1, 5);
    assert_eq!(v2, 2);
}
