use clap::{Parser, ValueEnum};
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "projgen", version, about = "Générateur de projet")]
struct Args {
    name: String,
    #[arg(value_enum)]
    template: TemplateType,
}

#[derive(Copy, Clone, ValueEnum, Debug)]
enum TemplateType {
    Rust,
    Web,
    Cli,
}

struct ProjectFile {
    path: String,
    content: String,
}
fn generate_project(root_dir: &str, files: Vec<ProjectFile>) -> io::Result<()> {
    let root_path = Path::new(root_dir);
    fs::create_dir_all(root_path)?;
    for file in files {
        let file_path = root_path.join(&file.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, file.content)?;
    }

    Ok(())
}

fn generate_template_files(t: TemplateType, name: &str) -> Vec<ProjectFile> {
    match t {
        TemplateType::Rust => vec![
            ProjectFile {
                path: "Cargo.toml".to_string(),
                content: format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", name),
            },
            ProjectFile {
                path: "src/main.rs".to_string(),
                content: "fn main() {\n    println!(\"Hello World!\");\n}".to_string(),
            },
            ProjectFile {
                path: "README.md".to_string(),
                content: format!("# Rust {}\nA simple Rust project.", name),
            },
            ProjectFile {
                path: ".gitignore".to_string(),
                content: "**/*.rs.bk \n".to_string(),
            }
        ],
        TemplateType::Web => vec![
            ProjectFile {
                path: "index.html".to_string(),
                content: format!("<html><head><title>{}</title></head><body></body></html>", name),
            },
            ProjectFile {
                path: "style.css".to_string(),
                content: "/* CSS */".to_string(),
            },
            ProjectFile {
                path: "script.js".to_string(),
                content: "// JavaScript".to_string(),
            },
            ProjectFile {
                path: "README.md".to_string(),
                content: format!("# WEB\nA simple web project for {}.", name),
            },
        ],
        TemplateType::Cli => vec![
            ProjectFile {
                path: "README.md".to_string(),
                content: format!("# CLI {}\nUsage: ./{} [options]", name, name),
            },
        ],
    }
}
fn git_init(project_path: &str ) -> io::Result<()> {
    let status = Command::new("git").arg("init").current_dir(project_path).status()?;
    if !status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize git repository"));
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    println!("Préparation du projet '{}'...", args.name);
    let files = generate_template_files(args.template, &args.name);
    if let Err(e) = generate_project(&args.name, files) {
        eprintln!("Error : {}", e);
        std::process::exit(1);
    }
    if let Err(e) = git_init(&args.name) {
        eprintln!("Erreur lors d'initialisation de git : {}", e);
        std::process::exit(1);
    }
    println!("Projet '{}' créé avec succès !", args.name);
}

