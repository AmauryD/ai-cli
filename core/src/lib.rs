
use configuration::Configuration;
use context::Context;
use gpt_call::GptCallTrait;
use gpt_error::GptError;

use crate::gpt_call::GptCall;

mod ureq_mock;
mod gpt_call;

pub mod configuration;
pub mod gpt_response;
pub mod gpt_error;
pub mod context;

pub fn completion(ask: &str, config: Configuration, context: Context) -> Result<String, GptError> {
    let gpt_call = GptCall::new(config, context);

    match gpt_call.call_gpt(ask).map_err(|_| GptError) {
        Ok(response) => Ok(response.choices.first().unwrap().message.content.clone()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
}
