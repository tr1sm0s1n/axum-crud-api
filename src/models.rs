use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Certificate {
    pub id: u32,
    name: String,
    course: String,
    status: bool,
    date: Option<String>,
}
