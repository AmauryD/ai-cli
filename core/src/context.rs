pub struct Context<'a> {
    pub env_vars: &'a str,
    pub cwd: &'a str,
    pub available_commands: &'a str,
}