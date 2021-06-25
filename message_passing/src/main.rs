// mpsc stands for multiple producer, single consumer.
// In Rust, you can have many upstream producers of messages,
// but only a single downstream consumer.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Access the transmitter (Sender) and a receiver (Receiver) for a channel.
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and use the move keyword to signal we're moving
    // ownership of tx to the thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("Hey"),
            String::from("Parkie"),
            String::from("Doo"),
        ];

        for message in vals {
            // tx.send() returns a Result<T, Err>. Using unwrap here would _not_ handle
            // the Err case, i.e. if the receiving end has been dropped.
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive the messages on the main thread via the receiver's recv method.
    // This will block the main thread until a message is received. try_recv is
    // non-blocking and immediately returns a Result<T, Err>.
    for message in rx {
        println!("Got: {}", message);
    }
}
