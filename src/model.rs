use serde::Deserialize;
use std::fmt;
use std::fs;
use colored::*;
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize)]
pub struct CommandEntry {
    pub name: String,
    pub description: String,
}

pub enum Entry {
    Directory(DirectoryEntry),
    Command(CommandEntry),
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name.blue().bold())
    }
}

impl fmt::Display for CommandEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t{}", self.name , self.description)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entry::Directory(directory_entry) => write!(f, "{}", directory_entry),
            Entry::Command(command_entry) => write!(f, "{}", command_entry),
        }
    }
}

pub fn get_entries(directory_path: &str) -> Result<Vec<Entry>, std::io::Error> {
    let dir_entries = fs::read_dir(directory_path)?;

    let mut entries: Vec<Entry> = Vec::new();

    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            entries.push(Entry::Directory(DirectoryEntry {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.to_str().unwrap().to_string(),
            }));
        } else {
            match path.extension() {
                Some(ext) => {
                    if ext == "csv" {
                        let csv = fs::read_to_string(path).expect("Failed to read file");

                        let mut reader = csv::Reader::from_reader(csv.as_bytes());

                        for record in reader.deserialize() {
                            let entry: CommandEntry = record?;
                            entries.push(Entry::Command(entry));
                        }
                    }
                }
                None => continue,
            }
        }
    }

    Ok(entries)
}
