fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let home_u8 = IpAddrKind::V4U8(127, 0, 0, 1);

    let message = Message::Write(String::from("Hey Doo!"));
    message.call();

    // One of the most important enum types in Rust is the Option enum.
    let my_int_option = Some(5);

    // Note that you must define the generic type when instantiating a None value.
    // Rust cannot infer the generic type T from None alone.
    let my_other_int_option: Option<i32> = None;
}

// An enum type, defining possible variants of an IP address.
// Enum members can accept data, which is powerful for associating data with a type.
enum IpAddrKind {
    V4(String),
    V6(String),
    // Enum members can also accept _different_ data types.
    V4U8(u8, u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling a method on an enum! {:#?}", &self)
    }
}
