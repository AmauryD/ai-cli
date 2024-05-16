use std::fmt;

#[derive(Debug, Clone)]
pub struct GptError;

impl fmt::Display for GptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while calling the GPT API")
    }
}