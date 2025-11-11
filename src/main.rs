use std::io;

fn main() {
    println!("Hello! What's your name ?");

    let mut your_name = String::new();

    // obtain keyboard input and store it as a string
    io::stdin()
        .read_line(&mut your_name)
        .expect("Failed! to read the line");
    println!("Hello {}", your_name)
}
