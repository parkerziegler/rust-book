use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // The use of the move keyword before the closure is critical here.
    // move signals that the thread is taking ownership over v. Without
    // move, we'd get a compile-time error that the thread may outlive
    // the vector, v, preventing us from referencing it.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
