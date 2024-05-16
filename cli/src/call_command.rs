use std::process::Command;

pub fn get_available_commands() -> Result<String, std::io::Error> {
    let output = Command::new("bash").args(&["-c", "compgen"])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}