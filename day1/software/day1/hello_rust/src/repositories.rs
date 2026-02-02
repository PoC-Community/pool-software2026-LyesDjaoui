pub mod tasks {
    use crate::models::Task;
    use std::fs;

    const FILE_PATH: &str = "data/tasks.json";

    pub fn save_all(tasks: Vec<Task>) {
        let data = serde_json::to_string_pretty(&tasks).expect("Erreur de sérialisation");
        fs::write(FILE_PATH, data).expect("Impossible d'écrire dans le fichier");
    }

    pub fn read_all() -> Vec<Task> {
        let data = fs::read_to_string(FILE_PATH).expect("Impossible de lire le fichier");
        return serde_json::from_str(&data).expect("Erreur de désérialisation");
    }    
}