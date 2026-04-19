use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub content: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub shell_type: Option<String>,
}

impl Snippet {
    pub fn new(
        content: String,
        tag: Option<String>,
        description: Option<String>,
        shell_type: Option<String>,
    ) -> Result<Self> {
        Ok(Snippet {
            id: Self::generate_id()?,
            content,
            tag,
            description,
            shell_type,
        })
    }

    fn generate_id() -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("System clock is before Unix epoch (this should never happen)")?
            .as_millis();

        Ok(format!("{:x}", now))
    }

    pub fn tag_or_default(&self) -> &str {
        self.tag.as_deref().unwrap_or_default()
    }

    pub fn desc_or_default(&self) -> &str {
        self.description.as_deref().unwrap_or_default()
    }

    pub fn display_tag(&self) -> String {
        let tag = self.tag_or_default().trim();
        if tag.is_empty() {
            "UNTAGGED".to_string()
        } else {
            tag.to_ascii_uppercase()
        }
    }
}
