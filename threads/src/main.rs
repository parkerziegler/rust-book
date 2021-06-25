use std::thread;
use std::time::Duration;

fn main() {
    // To spawn a new thread in Rust, use thread::spawn. Note that
    // all threads will be shut down when the main thread is shut down.
    // In the example below, this means that this spawned thread only
    // prints 4 or 5 times before the main thread is shut down.
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // Call thread::sleep to allow the main thread to run. It's
            // likely that the spawned and main threads will alternate,
            // though this is determined by the OS.
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
