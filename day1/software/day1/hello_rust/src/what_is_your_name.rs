use std::io;

pub fn what_is_your_name() -> String {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}