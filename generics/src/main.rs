fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_return_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    &largest
}

fn main() {
    let num_list = vec![11, 121, 89, 34, 47, 93, 3];

    println!("The largest number is: {}", largest(&num_list));

    let num_list = vec![89, 43, 57, 109, 92, 13, 22];

    println!("The largest number is: {}", largest(&num_list));

    let char_list = vec!['y', 'm', 'q', 'a'];

    println!("The largest character is: {}", largest(&char_list));

    let a = Point { x: 1, y: 2 };

    println!("a.x: {}", a.x());

    let b = Point { x: 1.0, y: 2.0 };

    let c = MixedPoint { x: 1, y: 2.0 };

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Generics can be used on structs to allow the same struct to store different types of data.
struct Point<T> {
    x: T,
    y: T,
}

// Methods in a struct can also use generics.
impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

// You can also only implement a method for a particular concrete type.
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// You can also define generics on a struct and on a particular method on that struct.
impl<T, U> MixedPoint<T, U> {
    pub fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Enums can also accept generics.
enum Status<T, U> {
    Ok(T),
    Uhoh(U),
}
