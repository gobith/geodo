mod clipboard;
mod model;
use clipboard::copy_to_clipboard;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use model::{get_entries , Entry};

fn main() -> std::io::Result<()> {
    show_entries("commands").expect("Failed to show entries");
    Ok(())
}

fn show_entries(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = get_entries(path).expect("Failed to get entries");

    entries.sort();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&entries)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            let entry = &entries[index];
            match entry {
                Entry::Directory(directory_entry) => show_entries(&directory_entry.path)?,
                Entry::Command(command_entry) => copy_to_clipboard(&command_entry.name).expect("Failed to copy to clipboard")
            }
        }
        None => println!("No item selected"),
    }

    Ok(())
}
