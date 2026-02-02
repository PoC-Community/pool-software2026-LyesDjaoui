pub mod tasks {
    use crate::models::{Task, Priority};
    use std::io;

    pub fn list_all(tasks: &[Task]) {
        println!("\nHere are you tasks :");
        for task in tasks {
            let mut status = " ";
            if task.completed {
                status = "x";
            }
            println!("-- {} -- [{}] {} - Priority: {:?} - Created at: {} - Tags: {}\n", task.id, status, task.description, task.priority, task.created_at, task.tags.join(", "));
        }
    }

    pub fn prompt_add() -> (String, Priority) { 
        println!("Entrez la description de la tache :");
        let mut desc_input = String::new();
        io::stdin().read_line(&mut desc_input).unwrap();
        let description = desc_input.trim().to_string();

        println!("Entrez la priorité (1: Low, 2: Medium, 3: High) :");
        let mut prio_input = String::new();
        io::stdin().read_line(&mut prio_input).unwrap();
    
        let priority = match prio_input.trim() {
            "1" | "Low" => Priority::Low,
            "2" | "Medium" => Priority::Medium,
            "3" | "High" => Priority::High,
            _ => Priority::Low,
    };

    (description, priority)
}

    pub fn prompt_task_id() -> u32 {
        println!("Entrez l'id de la tache à supprimer :");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide")
    }


    pub fn prompt_update_task() -> u32 {
        println!("\nWhat do you want to update ?");
        println!("1 - Mark as completed");
        println!("2 - Update the task description");
        println!("3 - Update the task priority");
        println!("4 - Update the task tags");
        println!("5 - Leave");
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide")
    }


    pub fn prompt_update_task_completed() -> u32 {
        println!("Enter the id of the task to mark as completed :");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide")
    }


    pub fn prompt_update_task_description() -> (u32, String) {
        println!("Enter the id of the task to update the description :");
        let mut id_input = String::new();
        std::io::stdin().read_line(&mut id_input).unwrap();
        let id = id_input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide");

        println!("Enter the new description :");
        let mut desc_input = String::new();
        std::io::stdin().read_line(&mut desc_input).unwrap();
        let description = desc_input.trim().to_string();

        (id, description)
    }

    pub fn prompt_update_task_priority() -> (u32, Priority) {
    println!("Enter the id of the task to update the priority :");
    let mut id_input = String::new();
    std::io::stdin().read_line(&mut id_input).unwrap();
    let id = id_input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide");

    println!("Enter the new priority (1: Low, 2: Medium, 3: High) :");
    let mut priority_input = String::new();
    std::io::stdin().read_line(&mut priority_input).unwrap();
    
    let priority = match priority_input.trim() {
        "1" | "Low" => Priority::Low,
        "2" | "Medium" => Priority::Medium,
        "3" | "High" => Priority::High,
        _ => {
            println!("Choix invalide , priorité par défaut Low attribuée.");
            Priority::Low
        }
    };
    (id, priority)
}

    pub fn prompt_update_task_tags() -> (u32, Vec<String>) {
        println!("Enter the id of the task to update the tags :");
        let mut id_input = String::new();
        std::io::stdin().read_line(&mut id_input).unwrap();
        let id = id_input.trim().parse::<u32>().expect("Veuillez entrer un nombre valide");

        println!("Enter the new tags :");
        let mut tags_input = String::new();
        std::io::stdin().read_line(&mut tags_input).unwrap();
        let tags: Vec<String> = tags_input
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        (id, tags)
    }
}