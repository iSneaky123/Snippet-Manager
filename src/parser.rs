use std::{collections::HashSet, path::Path};

pub struct ArgParser<'a> {
    pub args: Vec<String>,
    bool_flags: HashSet<&'a str>,
    value_flags: HashSet<&'a str>,
}

impl<'a> ArgParser<'a> {
    pub fn new() -> Self {
        Self {
            args: std::env::args().collect(),
            bool_flags: HashSet::new(),
            value_flags: HashSet::new(),
        }
    }

    pub fn bin_name(&self) -> String {
        self.args
            .first()
            .and_then(|s| Path::new(s).file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("snip")
            .to_string()
    }

    pub fn cmd(&self) -> Option<&str> {
        self.args.get(1).map(|s| s.as_str())
    }

    pub fn has_flag(&mut self, short: &'a str, long: &'a str) -> bool {
        // register the flag
        self.bool_flags.insert(short);
        self.bool_flags.insert(long);

        self.args.iter().any(|flag| flag == short || flag == long)
    }

    pub fn get_value(&mut self, short: &'a str, long: &'a str) -> Result<Option<String>, String> {
        // register the flag
        self.value_flags.insert(short);
        self.value_flags.insert(long);

        if let Some(pos) = self
            .args
            .iter()
            .position(|flag| flag == short || flag == long)
        {
            match self.args.get(pos + 1) {
                Some(val) if !val.starts_with('-') => Ok(Some(val.clone())),
                _ => Err(format!("Error: Flag {}/{} requires a value!", short, long)),
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_content(&self) -> Result<String, String> {
        let mut content: Vec<String> = vec![];
        let mut i = 2; // Skip Binary Name and command

        while i < self.args.len() {
            let arg = &self.args[i];
            if arg.starts_with('-') {
                if self.bool_flags.contains(arg.as_str()) {
                    i += 1;
                } else if self.value_flags.contains(arg.as_str()) {
                    i += 2;
                } else {
                    return Err(format!("Invalid argument {arg}"));
                }
            } else {
                content.push(arg.clone());
                i += 1;
            }
        }
        Ok(content.join(" ").trim().to_string())
    }
}
