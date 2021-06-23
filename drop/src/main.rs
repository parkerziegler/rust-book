use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

// Below we implement the Drop trait for the CustomSmartPointer struct.
// When an instance of CustomSmartPointer goes out of scope, the drop
// function defined on the Drop trait will get run. The Drop trait is
// included in the prelude so there's no need to use it. Variables are
// dropped in the reverse order that they were created.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointr with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("yet more stuff"),
    };

    println!("CustomSmartPointers created!");

    // We can use the drop function from std::mem::drop to prematurely
    // drop a value before it goes out of scope.
    drop(e);

    println!("CustomSmartPointer dropped before the end of main.");
}
