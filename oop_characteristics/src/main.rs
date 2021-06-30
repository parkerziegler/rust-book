// Structs are similar to objects in traditional OOP in that they
// have both data (fields on the struct) and procedures that operate
// on that data (methods).
//
// The struct below has two private fields, list and average.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// The impl block is where we define the methods on a struct. Notice that
// the add, remove, and average methods have the pub keyword to indicate
// they're part of the public interface. update_average() is private and
// can only be called by methods of the struct.
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let average = self.list.iter().sum() as f64 / self.list.len() as f64;
        self.average = average;
    }
}

fn main() {
    println!("Hello, world!");
}
