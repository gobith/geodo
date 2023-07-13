use std::io::Write;
use std::process::Command;

pub fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
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