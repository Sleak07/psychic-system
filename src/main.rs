use std::io;



// Putting the asking name oportion inside a custom name function
fn what_is_your_name() -> String {
    let mut your_name = String::new();

    // obtain keyboard input and store it as a string
    io::stdin()
        .read_line(&mut your_name)
        .expect("Failed! to read the line");
    your_name.trim().to_lowercase();
    
    let visitor_list = ["steve","bert","stew"];
    let mut allow_them_in = false;
    for visitor in &visitor_list{
        if visitor == &name 
    }


}


fn main() {
    println!("Hello! What's your name ?");
    let name = what_is_your_name();
    println!("Hello {}", name)
}
