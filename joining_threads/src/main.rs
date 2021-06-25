use std::thread;
use std::time::Duration;

fn main() {
    // To ensure a spawned thread completes even when the main
    // thread has finished, we can use a JoinHandle. Store the
    // result of thread::spawn in a let binding, and then call the
    // .join() method on it. This will ensure we wait for this thread
    // to finish before proceeding.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Note that calling .join() on the handle will block the main thread
    // completely until the spawned thread completes. If we wanted the for
    // loop on L17 to wait completely for the spawned thread to finish, we
    // could move this call above it.
    handle.join().unwrap();
}
