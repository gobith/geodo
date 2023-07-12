use std::fmt;
pub struct DirectoryEntry {
    pub name: String,
    pub path: String,
}

pub struct CommandEntry {
    pub name: String,
    pub description: String,
}

pub trait Entry {
    fn name(&self) -> &str;
    fn color(&self) -> &str;
}

impl Entry for DirectoryEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> &str {
        "blue"
    }
}

impl Entry for CommandEntry {
    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> &str {
        "green"
    }
}

impl fmt::Display for dyn Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}