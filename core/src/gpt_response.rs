use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GptResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize,Debug)]
pub struct Choice {
    pub message: Message,
}

#[derive(Deserialize,Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    // no logic to test
}
