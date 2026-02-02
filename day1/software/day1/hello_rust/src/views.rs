pub mod tasks {
    use crate::models::Task;

    pub fn list_all(tasks: &[Task]) {
        println!("\nHere are you tasks :");
        for task in tasks {
            let mut status = " ";
            if task.completed {
                status = "x";
            }
            println!("-- {} -- [{}] {} \n", task.id, status, task.description);
        }
    }

    pub fn prompt_description() -> String {
        println!("Entrez la description de la tache :");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
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
        println!("3 - Leave");
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
}