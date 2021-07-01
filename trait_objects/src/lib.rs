// Draw is a trait, defining the interface that must be
// fulfilled by any type which implements it. In this case,
// Draw only requires one method, draw, to be implemented.
pub trait Draw {
    fn draw(&self);
}

// Below, components represents a vector of trait objects.
// The trait object Box<dyn Draw> indicates that any type
// implementing the Draw trait will be allowed in this vector.
//
// Generic traits only allow one concrete type to be substituted
// for the generic parameter; conversely, trait objects allow for
// multiple concrete types to fill in for the trait object at runtime.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Here we implement the Draw trait on the Button struct.
impl Draw for Button {
    fn draw(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
