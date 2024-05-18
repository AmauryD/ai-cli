use std::fmt;

#[derive(Debug, Clone)]
pub enum  GptError {
    FailedToCallGpt,
}

impl fmt::Display for GptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while calling the GPT API")
    }
}

#[cfg(test)]
mod tests {
    // no logic to test
}
