use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    id: u32,
    name: String,
    course: String,
    status: bool,
    date: String,
}
