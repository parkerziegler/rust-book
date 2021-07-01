use trait_objects::{Button, Draw, Screen};

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

// Here, we implement the Draw trait from our library on a
// custom component (SelectBox). This demonstrates how a
// consumer of the lib may make use of the Draw trait to supply
// custom functionality that the lib doesn't implement.
impl Draw for SelectBox {
    fn draw(&self) {
        // Code to draw a SelectBox.
    }
}

fn main() {
    // The use of trait objects below is similar to duck typing in dynamic languages.
    // Screen doesn't care about the type of components it gets, just that they
    // implement the Draw trait and have a run method that can be called.
    //
    // The drawback of using trait objects is that they require dynamic dispatch.
    // The compiler doesn't know all the types that might be used with the code using
    // trait objects, so it needs to add additional code at runtime to inspect the
    // pointers inside of the trait objects.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 20,
                options: vec![
                    String::from("Banana"),
                    String::from("Papaya"),
                    String::from("Raspberry"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("Click Me"),
            }),
        ],
    };

    screen.run();
}
