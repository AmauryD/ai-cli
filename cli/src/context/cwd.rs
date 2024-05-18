pub trait GetCwd {
    fn get_cwd(&self) -> Result<String, String>;
}

pub struct GetCwdImpl;

impl GetCwd for GetCwdImpl {
    fn get_cwd(&self) -> Result<String, String> {
        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
        Ok(cwd.to_str().unwrap().to_string())
    }
}