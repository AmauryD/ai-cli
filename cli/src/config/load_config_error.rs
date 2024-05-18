#[derive(Debug)]
pub enum LoadConfigError {
    ConfigDoesNotExists,
    FailedToGetHome,
    CannotRead,
    FailedToParse,
}

impl std::fmt::Display for LoadConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadConfigError::ConfigDoesNotExists => write!(f, "Configuration file does not exists"),
            LoadConfigError::FailedToGetHome => write!(f, "Failed to get home directory"),
            LoadConfigError::CannotRead => write!(f, "Cannot read configuration file"),
            LoadConfigError::FailedToParse => write!(f, "Failed to parse configuration file"),
        }
    }
}
