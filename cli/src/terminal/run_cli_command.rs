use std::process::Command;

pub trait RunCommand {
    fn run_command(&self, command: String) -> Result<String, String>;
    fn run_silent(&self, command: String) -> Result<String, String>;
}

pub enum ShellType {
    Zsh
}

impl ShellType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ShellType::Zsh => "zsh"
        }
    }
}

pub struct ShellRunner {
    shell_type: ShellType,
}

impl ShellRunner {
    pub fn new(shell_type: ShellType) -> Self {
        ShellRunner { shell_type }
    }
}

impl RunCommand for ShellRunner {
    fn run_command(&self, command: String) -> Result<String, String> {
        let out = Command::new(self.shell_type.as_str())
            .args(&["-c", &command])
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .output().map_err(|e| e.to_string())?;
        
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }
    fn run_silent(&self, command: String) -> Result<String, String> {
        let out = Command::new(self.shell_type.as_str())
            .args(&["-c", &command])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output().map_err(|e| e.to_string())?;

        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }
}
