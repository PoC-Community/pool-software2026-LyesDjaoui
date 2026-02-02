use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)] 
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)] 
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}