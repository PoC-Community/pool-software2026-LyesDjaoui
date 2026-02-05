use clap::Parser;
use colored::*;
use std::fs;
use std::process;

#[derive(Parser, Debug)]
struct Args {
    pattern: String,
    file: String,

    #[arg(short = 'i', long = "case-insensitive")]
    case_insensitive: bool,

    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,
}
fn search_in_file(args: &Args) -> Result<() , Box<dyn std::error::Error>> {
    let content = fs::read_to_string(&args.file)?;
    let mut found_match = false;

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        let matches = if args.case_insensitive {
            line.to_lowercase().contains(&args.pattern.to_lowercase())
        } else {
            line.contains(&args.pattern)
        };

        if matches {
            found_match = true;
            let highlighted_line = highlight_pattern(line, &args.pattern, args.case_insensitive);

            if args.line_numbers {
                println!("{}:{}", line_number.to_string().green().bold(), highlighted_line);
            } else {
                println!("{}", highlighted_line);
            }
        }
    }

    if !found_match {
        println!("Aucune correspondance trouver pour '{}'", args.pattern);
    }

    Ok(())
}

fn highlight_pattern(line: &str, pattern: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        let mut result = String::new();
        let line_lower = line.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        let mut last_pos = 0;

        for (pos, matched) in line_lower.match_indices(&pattern_lower) {
            result.push_str(&line[last_pos..pos]);
            result.push_str(&line[pos..pos + matched.len()].red().bold().to_string());
            last_pos = pos + matched.len();
        }
        result.push_str(&line[last_pos..]);
        result
    } else {
        line.replace(pattern, &pattern.red().bold().to_string())
    }
}

fn main() {
    let args = Args::parse();
    search_in_file(&args);
    }


