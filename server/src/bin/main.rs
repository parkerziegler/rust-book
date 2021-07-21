use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Bind the TcpListener to local IP on port 7878. The bind function
    // is similar to ::new in that it returns a new instance of a TcpListener.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The incoming method returns an iterator of TcpStreams. A single stream
    // is a connection between client and server. A connection is the name for the
    // full request / response process.
    //
    // To simulate graceful shutdown, we only take two incoming TCP connections.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Spawn a new connection for each incoming request (connection).
        // This allows a new thread to handle each connection.
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// Note that we need to mark this parameter as mutable because a TcpStream instance
// internally mutates the data it returns to us.
fn handle_connection(mut stream: TcpStream) {
    // Declare a buffer 1024 bytes in length. Handling requests of an arbitrary size
    // is a more complex process.
    let mut buffer = [0; 1024];

    // Read bytes from the stream and put them in the buffer.
    stream.read(&mut buffer).unwrap();

    // Print the request as a String. The lossy suffix below indicates that Rust should
    // use the U+FFDD REPLACEMENT CHARACTER, �, when it encounters invalid UTF-8.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // b transforms the string literal into a byte string.
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Ensure that the request is made only to /, otherwise 404.
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\n Content-Length: {} \r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write the response back to the stream.
    stream.write(response.as_bytes()).unwrap();

    // Calling the flush method will prevent the program from continuing until all
    // bytes are written to the connection.
    stream.flush().unwrap();
}

// HTTP is a text-based protocol, with a request taking the form:
//
// Method Request-URI HTTP-version CRLF
// headers CRLF
// message-body
//
// CRLF = carriage return line feed
//
// Responses in HTTP take the following form:
//
// HTTP-version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
