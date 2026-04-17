use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub content: String,
    pub tag: String,
    pub description: String,
    pub shell_type: String,
}

impl Snippet {
    pub fn new(content: String, tag: String, description: String, shell_type: String) -> Self {
        Snippet {
            id: Self::generate_id(),
            content,
            tag,
            description,
            shell_type,
        }
    }

    fn generate_id() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        format!("{:x}", now)
    }
}
