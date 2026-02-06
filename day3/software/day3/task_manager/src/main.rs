use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::time::SystemTime;

#[derive(Parser)]
#[command(name = "Tasky")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
        #[arg(short, long, value_enum, default_value_t = PriorityLevel::Medium)]
        priority: PriorityLevel,
    },
    List {
        #[arg(short, long)]
        completed: Option<bool>,
    },
    Complete {
        id: u32,
    },
    Delete {
        id: u32,
    },
    Priority {
        id: u32,
        #[arg(value_enum)]
        level: PriorityLevel,
    },
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum PriorityLevel {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    priority: PriorityLevel,
    created_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskStore {
    fn new() -> Self {
        TaskStore {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String, priority: PriorityLevel) {
        let new_task = Task {
            id: self.next_id,
            description,
            completed: false,
            priority,
            created_at: Some(SystemTime::now()),
        };
        self.tasks.push(new_task);
        self.next_id += 1;
        println!("{}", "tache ajoutée !".green());
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("{}", "Aucune Tache trouvé, le repos fait parti du travail les brother".yellow());
            return;
        }

        println!("\n{}", "=== MES TÂCHES ===".bold());
        for task in &self.tasks {
            let status = if task.completed { "DONE".green() } else { "TODO".red() };
            let prio = match task.priority {
                PriorityLevel::High => "HIGH".red().bold(),
                PriorityLevel::Medium => "MEDIUM".yellow(),
                PriorityLevel::Low => "LOW".blue(),
            };

            println!(
                "\n[{}] {} | {} | {}",
                task.id.to_string().cyan().bold(),
                task.description,
                status,
                prio
            );
        }
        println!();
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Ok(())
        } else {
            Err(format!("ID {} introuvable", id))
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() < initial_len { Ok(()) } else { Err(format!("ID {} introuvable", id)) }
    }

    fn update_priority(&mut self, id: u32, level: PriorityLevel) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.priority = level;
            Ok(())
        } else {
            Err(format!("ID {} introuvable", id))
        }
    }
}

fn load_tasks(file_path: &str) -> TaskStore {
    if !Path::new(file_path).exists() {
        return TaskStore::new();
    }
    let file = File::open(file_path).unwrap_or_else(|_| return File::create(file_path).unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| TaskStore::new())
}

fn save_tasks(tasks: &TaskStore, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    let json = serde_json::to_string_pretty(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let file_path = "tasks.json";
    let mut store = load_tasks(file_path);

    match cli.command {
        Commands::Add { description, priority } => {
            store.add_task(description, priority);
        }
        Commands::List { .. } => {
            store.list_tasks();
        }
        Commands::Complete { id } => {
            if let Err(e) = store.complete_task(id) { eprintln!("{}", e.red()); }
        }
        Commands::Delete { id } => {
            if let Err(e) = store.delete_task(id) { eprintln!("{}", e.red()); }
        }
        Commands::Priority { id, level } => {
            if let Err(e) = store.update_priority(id, level) { eprintln!("{}", e.red()); }
        }
    }

    save_tasks(&store, file_path).expect("Erreur de sauvegarde");
}