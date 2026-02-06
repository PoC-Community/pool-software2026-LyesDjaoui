use anyhow::{Context, Result, anyhow};
use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::collections::BTreeSet;

#[derive(Parser)]
#[command(author, version, about = "Convertisseur de fichiers (JSON, YAML, CSV)")]
struct Cli {
    input: String,

    output: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Format {
    Json,
    Yaml,
    Csv,
}

impl Format {
    fn detect_format(path: &str) -> Result<Self> {
        let extension = Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "json" => Ok(Format::Json),
            "yaml" | "yml" => Ok(Format::Yaml),
            "csv" => Ok(Format::Csv),
            _ => Err(anyhow!("Extension non supportée ou inconnue pour : {}", path)),
        }
    }
}

fn parse_file(path: &str, format: Format) -> Result<Value> {
    let file = File::open(path).with_context(|| format!("Impossible d'ouvrir le fichier "))?;
    let reader = BufReader::new(file);

    match format {
        Format::Json => {
            let data: Value = serde_json::from_reader(reader)
                .context("Erreur lors du parsing JSON")?;
            Ok(data)
        }
        Format::Yaml => {
            let data: Value = serde_yaml::from_reader(reader)
                .context("Erreur lors du parsing YAML")?;
            Ok(data)
        }
        Format::Csv => {
            let mut rdr = csv::Reader::from_reader(reader);
            let mut rows = Vec::new();
            
            for result in rdr.deserialize() {
                let record: serde_json::Map<String, Value> = result
                    .context("Erreur lors de la lecture d'une ligne CSV")?;
                rows.push(Value::Object(record));
            }
            Ok(Value::Array(rows))
        }
    }
}

fn convert_format(path: &str, format: Format, data: &Value) -> Result<()> {
    let file = File::create(path).with_context(|| format!("Impossible de créer le fichier : {}", path))?;
    let writer = BufWriter::new(file);

    match format {
        Format::Json => {
            serde_json::to_writer_pretty(writer, data)
                .context("Erreur lors de l'écriture JSON")?;
        }
        Format::Yaml => {
            serde_yaml::to_writer(writer, data)
                .context("Erreur lors de l'écriture YAML")?;
        }
        Format::Csv => {
            let mut wtr = csv::Writer::from_writer(writer);
            
            let rows = data.as_array().ok_or_else(|| anyhow!("Erreur lors de la converstin"))?;

            if rows.is_empty() {
                return Ok(());
            }

            let mut headers: BTreeSet<String> = BTreeSet::new();
            for row in rows {
                if let Some(obj) = row.as_object() {
                    for key in obj.keys() {
                        headers.insert(key.clone());
                    }
                }
            }

            let headers_vec: Vec<&String> = headers.iter().collect();
            wtr.write_record(&headers_vec).context("Erreur lors de l'ecriture des headers Csv")?;

            for row in rows {
                if let Some(obj) = row.as_object() {
                    let record: Vec<String> = headers_vec.iter().map(|header| {
                        match obj.get(*header) {
                            Some(Value::String(s)) => s.clone(),
                            Some(Value::Null) => String::new(), 
                            Some(v) => v.to_string(), 
                            None => String::new()
                        }
                    }).collect();
                    
                    wtr.write_record(&record).context("Erreur lors de l'ecriture d'une ligne Csv")?;
                }
            }
            wtr.flush().context("Erreur lors du flush du buffer CSV")?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_format = Format::detect_format(&args.input)?;
    let output_format = Format::detect_format(&args.output)?;

    println!("Conversion : {} ({:?}) -> {} ({:?})", 
             args.input, input_format, args.output, output_format);
    let data = parse_file(&args.input, input_format)?;
    convert_format(&args.output, output_format, &data)?;

    println!("Fichier converti");
    Ok(())
}