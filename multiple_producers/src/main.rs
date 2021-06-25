use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Call clone on the Sender to create a second sender we can use to pass
    // messages from in the spawned thread.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            String::from("!"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Print all messages received from the two threads spawned above.
    // These will appear in a non-deterministic order.
    for message in rx {
        println!("Got message: {}", message);
    }
}
