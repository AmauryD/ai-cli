use gpt_call::GptCall;
use gpt_error::GptError;

pub mod gpt_call;
pub mod configuration;
pub mod gpt_response;
pub mod gpt_error;
pub mod context;

pub fn call_gpt<T: GptCall>(ask: &str, caller: T) -> Result<String, GptError> {
    match caller.call_gpt(ask).map_err(|_| GptError::FailedToCallGpt) {
        Ok(response) => Ok(response.choices.first().unwrap().message.content.clone().replace("`", "")),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
}   
