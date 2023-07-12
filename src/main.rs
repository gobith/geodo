use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::fs;
use std::io::Write;
use std::process::Command;

mod model;
use model::{CommandEntry, DirectoryEntry, Entry};

fn main() -> std::io::Result<()> {
    let entries = get_entries("commands").expect("Failed to get entries");
    for entry in &entries {
        println!("{}", entry.name());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&entries)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            copy_to_clipboard(&entries[index].name()).expect("Failed to copy to clipboard")
        }
        None => println!("No item selected"),
    }

    Ok(())
}

fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
    let output = Command::new("echo")
        .arg("-n")
        .arg(text)
        .output()
        .expect("Failed to execute command.");

    if output.status.success() {
        let copy_command = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command.");

        copy_command
            .stdin
            .expect("Failed to open stdin")
            .write_all(&output.stdout)
            .expect("Failed to write to stdin");
    } else {
        panic!("Failed to execute echo command.");
    }

    Ok(())
}

fn get_entries(directory_path: &str) -> Result<Vec<Box<dyn Entry>>, std::io::Error> {
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
        }
    }

    Ok(entries)
}
