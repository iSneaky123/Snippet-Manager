use serde::{Deserialize, Serialize};
use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::errors::{Result, SnipError};

/// Represents a Snippet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snippet {
    pub id: String,
    pub content: String,
    pub tag: Option<String>,
    pub description: Option<String>,
    pub shell: Option<Shell>,
}

/// Represents a shell that can execute commands
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shell {
    pub name: String,
    pub command_flag: String,
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
            .map_err(|_| {
                SnipError::Other(
                    "System clock is before Unix epoch (this should never happen)".to_string(),
                )
            })?
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
    /// Create a shell without validation.
    ///
    /// This is fast (no I/O) and always succeeds. Use this when you want
    /// a shell type but don't need to validate it exists yet.
    ///
    /// To verify the shell actually exists call `validate_async()` later.
    pub fn new_unchecked(name: String) -> Self {
        let command_flag = Self::get_command_flag(&name);
        Self { name, command_flag }
    }

    /// Get the command flag for a given shell name.
    ///
    /// Examples:
    /// - bash, sh, zsh: "-c"
    /// - cmd, cmd.exe: "/c"
    /// - powershell: "-Command"
    pub fn get_command_flag(name: &str) -> String {
        match name.to_lowercase().as_str() {
            "cmd" | "cmd.exe" => "/c".to_string(),
            "powershell" | "powershell.exe" => "-Command".to_string(),
            _ => "-c".to_string(),
        }
    }

    /// Validate that the shell exists and is executable.
    ///
    /// This spaws a subprocess to check if the shell binary is available.
    /// It's expensive, so it has been seperated from construction.
    pub fn validate(&self) -> bool {
        let mut cmd = Command::new(&self.name);
        cmd.arg(&self.command_flag).arg("");

        match cmd.output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}
