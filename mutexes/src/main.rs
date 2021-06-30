// Mutexes (an abbreviation for mutual exclusion) allow only one thread
// to access data at a time. Threads must request a lock on the data before
// accessing it and must release the lock when they're done with the data.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a new Mutex with Mutex::new().
    let m = Mutex::new(5);

    {
        // Request the lock on the mutex. This returns a Result<T, Err> type,
        // an Err being raised if a lock cannot be acquired. Note that acuiring
        // the lock is a blocking operation. Also note that after calling unwrap
        // we get a MutexGuard type, which implements the Deref and Drop traits.
        // This means that the mutex lock will be freed when the MutexGuard goes
        // out of scope.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // The Arc type allows us to reference count across multiple threads.
    // It's similar to Rc, but intended for use in multi-threaded contexts.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter result: {}", *counter.lock().unwrap());
}
