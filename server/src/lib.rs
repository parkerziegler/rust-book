use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a channel to pass jobs from the ThreadPool to individual
        // worker threads. Since mpsc architecture has _one consumer_, we need
        // to reference count the receiver across threads using a mutex.
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        // with_capacity preallocates the size of the vector, which is
        // more efficient than using Vec::new (dynamically resized).
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Create a new Worker for each thread. Ensure we bump the reference
            // count of the receiver using Arc::clone, allowing us to share the
            // receiver across multiple threads.
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    // Use FnOnce as the trait bound, which ensures that the closure
    // passed to execute can only run at most one time. Send allows for
    // the transfer of the closure from one thread to another and 'static
    // is useful because we don't know how long the thread will take to execute.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // Send the job down the channel to be processed by an available Worker.
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            // Attempt to obtain the lock on the mutex, then pull a job from the queue.
            let job = rx.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing!", id);

            job();
        });

        Worker { id, handle }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
