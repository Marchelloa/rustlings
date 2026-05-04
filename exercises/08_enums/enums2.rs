#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize { width: u32, height: u32 },
    Move(Point),
    Echo(String),
    ChangeColor (u32, u32, u32),
    Quit
}

impl Message {
    fn call(&self) {
        match self {
            Message::Resize { width, height } => {
                println!("Resizing to {}x{}", width, height);
            }
            Message::Move(point) => {
                println!("Moving to x={}, y={}", point.x, point.y);
            }
            Message::Echo(text) => {
                println!("Echo: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            }
            Message::Quit => {
                println!("Quitting...");
            }
        }
    }
}

fn main() {
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
