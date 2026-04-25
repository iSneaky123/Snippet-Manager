use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snippet {
    pub id: String,
    pub content: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub shell: Option<Shell>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shell {
    pub name: String,
    pub command_flag: String,
    pub is_supported: bool,
}

impl Snippet {
    pub fn new(
        content: String,
        tag: Option<String>,
        description: Option<String>,
        shell: Option<Shell>,
    ) -> Result<Self> {
        Ok(Snippet {
            id: Self::generate_id()?,
            content,
            tag,
            description,
            shell,
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

impl Shell {
    pub fn new(name: String) -> Self {
        let command_flag = Self::get_command_flag(&name);
        let is_supported = Self::validate_shell(&name, &command_flag);
        Self {
            name,
            command_flag,
            is_supported,
        }
    }

    pub fn get_command_flag(name: &str) -> String {
        match name.to_lowercase().as_str() {
            "cmd" | "cmd.exe" => "/c".to_string(),
            "powershell" | "powershell.exe" => "-Command".to_string(),
            _ => "-c".to_string(),
        }
    }

    pub fn validate_shell(name: &str, command_flag: &str) -> bool {
        let mut cmd = Command::new(name);

        cmd.arg(command_flag).arg("");

        match cmd.output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}
