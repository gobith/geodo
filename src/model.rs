use serde::Deserialize;
use std::fmt;
use std::fs;
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct CommandEntry {
    pub name: String,
    pub description: String,
}

pub trait Entry {
    fn name(&self) -> &str;
    fn color(&self) -> &str;
    fn is_directory(&self) -> bool;
    fn path(&self) -> String;
    fn description(&self) -> &str;
}

impl Entry for DirectoryEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> &str {
        "blue"
    }

    fn is_directory(&self) -> bool {
        true
    }

    fn path(&self) -> String {
        self.path.to_string()
    }

    fn description(&self) -> &str {
        "directory"
    }
}

impl Entry for CommandEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> &str {
        "green"
    }

    fn is_directory(&self) -> bool {
        false
    }

    fn path(&self) -> String {
        "".to_string()
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for dyn Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.name() , self.description())
    }
}

pub fn get_entries(directory_path: &str) -> Result<Vec<Box<dyn Entry>>, std::io::Error> {
    let dir_entries = fs::read_dir(directory_path)?;

    let mut entries: Vec<Box<dyn Entry>> = Vec::new();

    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            entries.push(Box::new(DirectoryEntry {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.to_str().unwrap().to_string(),
            }));
        } else {
            let csv = fs::read_to_string(path).expect("Failed to read file");

            let mut reader = csv::Reader::from_reader(csv.as_bytes());

            for record in reader.deserialize() {
                let entry: CommandEntry = record?;
                entries.push(Box::new(entry));
            }
        }
    }

    Ok(entries)
}
