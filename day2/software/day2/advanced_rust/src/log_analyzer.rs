use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use chrono::NaiveDate;

#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    
}

pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub level: LogLevel,
    pub message: String,
}

pub struct LogStatistics {
    pub total_lines: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub debug_count: usize,
}
impl Default for LogStatistics {
    fn default() -> Self {
        Self {
            total_lines: 0,
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            debug_count: 0,
        }
    }
}

impl LogEntry {
    pub fn parse(line : &str ) -> Option<Self> {
        let partis : Vec<&str> = line.split_whitespace().collect();
        if partis.len() < 4 {
            return None;
        }
        let datetime = format!("{} {}", partis[0], partis[1]);
        let timestamp = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S").ok()?;

        let level = match partis[2] {
            "[ERROR]" => LogLevel::Error,
            "[WARNING]" => LogLevel::Warning,
            "[INFO]" => LogLevel::Info,
            "[DEBUG]" => LogLevel::Debug,
            _ => return None,
        };
        let message = partis[3..].join(" ");
        Some(LogEntry {
            timestamp,
            level,
            message,
        })
    }

    pub fn file_reading(path : &str) -> Result<LogStatistics, io::Error> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut stats = LogStatistics::default();
        for (index, line_result) in reader.lines().enumerate() {
            let line_number = index + 1;
            match line_result {
                 Ok(line) => {
                    if let Some(entry) = LogEntry::parse(&line) {
                        stats.total_lines += 1;
                        match entry.level {
                            LogLevel::Error => stats.error_count += 1,
                            LogLevel::Warning => stats.warning_count += 1,
                            LogLevel::Info => stats.info_count += 1,
                            LogLevel::Debug => stats.debug_count += 1,
                    }
                } else {
                    eprintln!("Attention : Ligne {} mal formée et ignoré.", line_number);
                }
            }
            Err(e) => {
                eprintln!("Erreur de lecture à la ligne {} : {}", line_number, e);
            }
        }
    }

        Ok(stats)
    }

    pub fn generate_rapport(stats: &LogStatistics) -> String {
        format!(
            "Rapport d'analyse des logs :\n
            Total des lignes : {}\n
            Erreurs : {}\n
            Avertissements : {}\n
            Infos : {}\n
            Debugs : {}",
            stats.total_lines,
            stats.error_count,
            stats.warning_count,
            stats.info_count,
            stats.debug_count
        )
    }

    pub fn filter_logs(entries: Vec<LogEntry>, level_filter: Option<LogLevel>,date_filter: Option<NaiveDate>,) -> Vec<LogEntry> {
        entries.into_iter().filter(|entry| {
                let matches_level = match &level_filter {
                    Some(level) => &entry.level == level,
                    None => true,
                };

                let matches_date = match date_filter {
                    Some(date) => entry.timestamp.date() == date,
                    None => true,
                };

                matches_level && matches_date
            })
            .collect()
    }
}