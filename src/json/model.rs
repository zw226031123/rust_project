use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub hobbies: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
