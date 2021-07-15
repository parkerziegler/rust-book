use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Bind the TcpListener to local IP on port 7878. The bind function
    // is similar to ::new in that it returns a new instance of a TcpListener.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // The incoming method returns an iterator of TcpStreams. A single stream
    // is a connection between client and server. A connection is the name for the
    // full request / response process.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
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

    // Ensure that the request is made only to /, otherwise 404.
    let (status_line, filename) = if buffer.starts_with(get) {
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
