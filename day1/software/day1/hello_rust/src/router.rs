use std::io;
use crate::controllers::tasks;

pub fn run() {
    println!("Welcome to your Task Manager !");

    loop {
        println!("\nWhat do you want to do?");
        println!("1 - List all tasks");
        println!("2 - Add a task");
        println!("3 - Update a task");
        println!("4 - Delete a task");
        println!("5 - Leave");
        print!("> ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();

        if choice == "1" {
            tasks::list_tasks();
        } else if choice == "2" {
            tasks::add_task();
        } else if choice == "3" {
            tasks::update_task();
        } else if choice == "4" {
            tasks::delete_task();
        } else if choice == "5" {
            println!("see you !");
            break;
        } else {
            println!("\nPlease type 1, 2, 3, 4 or 5 !");
        }
    }
}