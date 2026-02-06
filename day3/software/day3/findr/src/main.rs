use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "findr")]
#[command(about = "Find files by name pattern", long_about = None)]
struct Args {
    #[arg(value_name = "NAME")]
    name: String,

    #[arg(short = 't', long = "type", value_name = "EXTENSION")]
    extension: Option<String>,

    #[arg(short = 'd', long = "dir", value_name = "DIRECTORY", default_value = ".")]
    directory: PathBuf,
}
fn search_files(args: &Args) {
    let mut found_count = 0;

    for entry in WalkDir::new(&args.directory).into_iter().filter_map(|e| e.ok()){
        if entry.file_type().is_file() {
            if matches_criteria(&entry, args) {
                println!("{}", entry.path().display());
                found_count += 1;
            }
        }
    }

    println!();
    println!("Found {} ", found_count);
}

fn matches_criteria(entry: &walkdir::DirEntry, args: &Args) -> bool {
    let path = entry.path();
    
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_lowercase(),
        None => return false,
    };

    if !file_name.contains(args.name.to_lowercase()) {
        return false;
    }

    if let Some(ext_filter) = args.extension {
        let ext_filter = ext_filter.to_lowercase();
        
        let file_ext = match path.extension() {
            Some(ext) => ext.to_string_lossy().to_lowercase(),
            None => return false,
        };

        if file_ext != ext_filter {
            return false;
        }
    }

    true
}

fn main() {
    let args = Args::parse();

    println!("Searching for: {}", args.name);
    
    if let Some(ext) = &args.extension {
        println!("Filtering by extension: {}", ext);
    }
    
    println!("Starting directory: {}", args.directory.display());
    println!();

    search_files(&args);
}

