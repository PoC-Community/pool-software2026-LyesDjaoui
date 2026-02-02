pub mod tasks {
    use crate::repositories::tasks as repo;
    use crate::views::tasks as view;
    use crate::models::Task;
    use chrono::Utc;

    pub fn list_tasks() {
        let tasks = repo::read_all();
        view::list_all(&tasks);
    }

    pub fn add_task() {
        let (description, priority) = view::prompt_add();
        let mut tasks = repo::read_all();
        
        let mut max_id = 0;
        for task in &tasks {
            if task.id > max_id {
                max_id = task.id;
            }
        }
        let new_id = max_id + 1;
        let new_task = Task { id: new_id, description, completed: false , priority , created_at: Utc::now(), tags: vec![] };
        
        tasks.push(new_task);
        repo::save_all(tasks);
        println!("\nTache ajoutée !");
    }
    pub fn clear_task(id: u32) {
        let mut tasks = repo::read_all();
        tasks.retain(|task| task.id != id);
        repo::save_all(tasks);
    }

    pub fn delete_task() {
        let task_id = view::prompt_task_id();
        clear_task(task_id);
        println!("\nTache supprimée !");
    }

    pub fn update_task() {
        list_tasks();
        let task_id = view::prompt_update_task();
        let mut tasks = repo::read_all();
        
        if task_id == 1 {
            let completed_task_id = view::prompt_update_task_completed();
            for task in &mut tasks {
                if task.id == completed_task_id {
                    task.completed = !task.completed;
                    println!("\nTache mise à jour !");
                    repo::save_all(tasks);
                    return; 
                }
            }
        } else if task_id == 2 {
            let (desc_task_id, new_description) = view::prompt_update_task_description();
            let mut found = false;
            for task in &mut tasks {
                if task.id == desc_task_id {
                    task.description = new_description;
                    found = true;
                    break; 
                }
            }
            if found {
                println!("\nTache mise à jour !");
                repo::save_all(tasks);
            }
        } else if task_id == 3 {
            let (priority_task_id, new_priority) = view::prompt_update_task_priority();
            let mut found = false;
            for task in &mut tasks {
                if task.id == priority_task_id {
                    task.priority = new_priority;
                    found = true;
                    break; 
                }
            }
            if found {
                println!("\nTache mise à jour !");
                repo::save_all(tasks);
            }
        } else if task_id == 4 {
            let (tags_task_id, new_tags) = view::prompt_update_task_tags();
            let mut found = false;
            for task in &mut tasks {
                if task.id == tags_task_id {
                    task.tags = new_tags;
                    found = true;
                    break; 
                }
            }
            if found {
                println!("\nTache mise à jour !");
                repo::save_all(tasks);
            }
        } else {
            println!("\nLeaving update menu.");
        }
    }

    
}