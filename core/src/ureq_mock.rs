use ureq::Error;

use crate::gpt_response::{Choice, GptResponse, Message};


pub struct Ureq {

}

// act as a module mock to replace ureq::post
impl Ureq {
    pub fn set(&self, uri: &str, content_type: &str) -> &Ureq {
        self
    }

    pub fn send_json(&self, json: serde_json::Value) -> Result<&Ureq, Error> {
        Ok(self)
    }

    pub fn into_json(&self) -> Result<GptResponse, Error> {
        Ok(
            GptResponse {
                choices: vec![
                    Choice {
                        message: Message {
                            role: "system".to_string(),
                            content: "You're a command line assistant. You help find the best command to the user input.".to_string()
                        }
                    }
                ]
            }
        )
    }
}

pub fn post(uri: &str) -> Ureq {
    Ureq {}
}