use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
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
            id: generate_uuid(),
            description,
            completed: false,
            created_at,
            created_at_date,
        }
    }

    pub fn is_today(&self) -> bool {
        let today = Utc::now().date_naive();
        let task_date = DateTime::from_timestamp(self.created_at as i64, 0)
            .unwrap()
            .date_naive();
        task_date == today
    }
}

pub fn generate_uuid() -> String {
    let mut rng = rand::thread_rng();
    let mut uuid_parts = Vec::with_capacity(36);

    for _ in 0..8 {
        uuid_parts.push(format!("{:x}", rng.gen::<u8>()));
    }
    uuid_parts.push("-".to_string());

    for _ in 0..4 {
        uuid_parts.push(format!("{:x}", rng.gen::<u8>()));
    }
    uuid_parts.push("-".to_string());

    for _ in 0..4 {
        uuid_parts.push(format!("{:x}", rng.gen::<u8>()));
    }
    uuid_parts.push("-".to_string());

    for _ in 0..4 {
        uuid_parts.push(format!("{:x}", rng.gen::<u8>()));
    }
    uuid_parts.push("-".to_string());

    for _ in 0..12 {
        uuid_parts.push(format!("{:x}", rng.gen::<u8>()));
    }

    uuid_parts.join("")
}
