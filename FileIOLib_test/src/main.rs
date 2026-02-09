use FileIOLib::*;

fn main() {
    println!("Hello, world!");
    
    let result= append_to_file("test.txt".to_string(), "I have been writen through a program!".to_string());

    let result2= read_file("test.txt".to_string());
}
