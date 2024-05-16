
use ureq::Error;

#[cfg(feature = "mockdata")]
use crate::ureq_mock::post;

#[cfg(not(feature = "mockdata"))]
use ureq::post;


use crate::{configuration::Configuration, context::Context, gpt_response::GptResponse};

pub trait GptCallTrait {
    fn call_gpt(&self, ask: &str) -> Result<GptResponse, Error>;
}

pub struct GptCall<'a> {
    config: Configuration<'a>,
    context: Context<'a>,
}



impl GptCall<'_> {
    pub fn new<'a>(config: Configuration<'a>, context: Context<'a>) -> GptCall<'a> {
        GptCall {
            config,
            context,
        }
    }
}

impl GptCallTrait for GptCall<'_> {
    fn call_gpt(&self, ask: &str) -> Result<GptResponse, Error> {
        let response: GptResponse = post(self.config.gpt_url)
            .set("Content-Type", "application/json")
            .send_json(serde_json::json!({
                "model": self.config.model,
                "messages": [
                    {
                        "role": "system",
                        "content": "You're a command line assistant. You help find the best terminal command. Don't ask user questions. Answer only with the best command."
                    },
                    {
                        "role": "user",
                        "content": format!("The available terminal commands are : {}", self.context.available_commands)
                    },
                    {
                        "role": "user",
                        "content": format!("My current directory is : {}", self.context.cwd)
                    },
                    {
                        "role": "user",
                        "content": format!("My environment is : {}", self.context.env_vars)
                    },
                    {
                        "role": "user",
                        "content": format!("How to {}", ask)
                    }
                ],
                "stream": false
            }))?.into_json()?;
        
        Ok(response)
    }
}




#[cfg(test)]
mod tests {
    use crate::{configuration::Configuration, gpt_call::GptCall};

    use super::GptCallTrait;
}