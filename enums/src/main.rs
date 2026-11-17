#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize { width: u8, height: u8 },
    Move(Point),
    Echo(String),
    ChangeColor(u16, u16, u16),
    Quit,
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    println!(
        "{:?}",
        Message::Resize {
            width: 8,
            height: 20,
        }
    );
    println!("{:?}", Message::Quit);
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
