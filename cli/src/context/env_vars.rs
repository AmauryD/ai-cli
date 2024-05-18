pub trait ExtractEnvVars {
    fn extract_env_vars(&self) -> String;
}

pub struct ExtractEnvVarsImpl;

impl ExtractEnvVars for ExtractEnvVarsImpl {
    fn extract_env_vars(&self) -> String {
        std::env::vars().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n")
    }
}