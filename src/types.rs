use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub created_at: u64, // Use a u64 to represent the timestamp
    pub created_at_date: String,
}

impl Task {
    pub fn new(description: String) -> Self {
        let now = SystemTime::now();
        let created_at = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let created_at_date = chrono::Local::now().format("%Y-%m-%d").to_string();

        Task {
            description,
            completed: false,
            created_at,
            created_at_date,
        }
    }
}
