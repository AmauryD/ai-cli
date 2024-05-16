mod call_command;

use core::{completion, configuration::Configuration, context::Context};
use call_command::get_available_commands;
use std::{env, io::{stderr, stdin, stdout, Write}, process::{exit, Command}};

fn main() {
    let args = std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let config = Configuration::new("http://localhost:8080/v1/chat/completions", "llama3-8b-instruct");
    let cwdbuff = env::current_dir().unwrap();
    let available_commands = get_available_commands().unwrap();
    let env_vars = env::vars().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n");

    let context: Context = Context {
        env_vars: env_vars.as_str(),
        cwd: cwdbuff.to_str().unwrap(),
        available_commands: available_commands.as_str()
    };

    let response = completion(&args, config, context).unwrap();

    println!("{}", response);
    println!("Do you want to run this command? (y/n)");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    if input.trim() != "y" {
        exit(0);
    }
    
    let stripped = response.replace("`", "");

    println!("Running command: {}", stripped);

    let result = Command::new("zsh")
        .args(&["-c", &stripped])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                stdout().write_all(&output.stdout).unwrap();
            } else {
                stderr().write_all(&output.stderr).unwrap();
            }
        }
        Err(e) => {
            stderr().write_all(&e.to_string().as_bytes()).unwrap();
            exit(1);
        }
    }
}
