pub struct Context {
    pub env_vars: String,
    pub cwd: String,
    pub last_shell_commands: String,
    pub available_commands: String,
    pub file_tree: String,
}

#[cfg(test)]
mod tests {
    // no logic to test
}
