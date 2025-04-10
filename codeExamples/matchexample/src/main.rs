use std::io;

fn main() {

    println!("Please enter a greeting");

    let mut name  = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    match name.trim()
    {
        "Hello" => println!("Hi, Nice to meet you!"),
        "Goodbye" => println!("See you later!"),
        _ => println!("Unknown")

    }
    
}
