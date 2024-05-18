use std::io::stdout;

use config::load_from_file::LoadConfigFromFile;
use context::{cwd::GetCwdImpl, env_vars::ExtractEnvVarsImpl, get_context::GetContextImpl};
use terminal::run_cli_command::{RunCommand, ShellRunner, ShellType};

pub(crate) mod config {
    pub(crate) mod load_from_file;
    pub(crate) mod load_config_error;
}
pub(crate) mod context {
    pub(super) mod get_context;
    pub(crate) mod env_vars;
    pub(crate) mod cwd;
}

pub(crate) mod terminal {
    pub(crate) mod run_cli_command;
    pub(crate) mod ask_user_confirmation;
}
mod gpt_resolver;
mod get_gpt_result;



fn main() {
    let shell_runner = ShellRunner::new(ShellType::Zsh);
    let context_getter = GetContextImpl {
        shell_runner: &shell_runner,
        env_vars_extractor: &ExtractEnvVarsImpl {},
        cwd_extractor: &GetCwdImpl {},
    };

    let gpt = gpt_resolver::resolve_gpt(
        LoadConfigFromFile,
        context_getter
    );

    if let Err(e) = gpt {
        eprintln!("{}", e);
        return;
    }

    let prompt = std::env::args().collect::<Vec<String>>()[1..].join(" ");

    let response = get_gpt_result::get_gpt_result(
        gpt.unwrap(),
        prompt,
        stdout(),
        std::io::stdin().lock()
    );

    match response {
        Ok(response) => {
            shell_runner.run_command(response)
                .map(|output| println!("{}", output))
                .unwrap_or_else(|e| eprintln!("{}", e));
        },
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }
}
