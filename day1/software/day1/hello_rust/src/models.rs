use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)] 
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
} 