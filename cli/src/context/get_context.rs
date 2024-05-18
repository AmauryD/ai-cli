
use core::context::Context;
use crate::terminal::run_cli_command::RunCommand;

use super::{cwd::GetCwd, env_vars::ExtractEnvVars};

pub trait GetContext {
    fn get_context(&self) -> Result<Context, String>;
}

pub struct GetContextImpl<'a> {
    pub(crate) shell_runner: &'a dyn RunCommand,
    pub(crate) env_vars_extractor: &'a dyn ExtractEnvVars,
    pub(crate) cwd_extractor: &'a dyn GetCwd,
}

impl GetContext for GetContextImpl<'_> {
    fn get_context(&self) -> Result<Context, String> {
        let cwdbuff = self.cwd_extractor.get_cwd();
        let available_commands = self.shell_runner.run_silent("compgen -c".into());
        let last_shell_commands = self.shell_runner.run_silent("history -10".into());
        let tree = self.shell_runner.run_silent("ls".into());
        let env_vars = self.env_vars_extractor.extract_env_vars();
    
        Ok(Context {
            last_shell_commands: last_shell_commands.unwrap_or_default(),
            env_vars,
            cwd: cwdbuff.unwrap_or_default(),
            file_tree: tree.unwrap_or_default(),
            available_commands: available_commands.unwrap_or_default(),
        })
    }
}