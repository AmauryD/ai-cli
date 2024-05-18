use core::{call_gpt, gpt_call::GptCall};
use crate::terminal::ask_user_confirmation::ask_user_confirmation;

pub fn get_gpt_result<T: GptCall>(
    gpt: T,
    prompt: String,
    stdout: impl std::io::Write,
    stdin: impl std::io::BufRead
) -> Result<String, String> {
    let response = call_gpt(prompt.as_str(), gpt).map_err(|e| e.to_string())?;

    println!("{}", response);

    if !ask_user_confirmation("Do you want to run this command ?".into(), stdout, stdin) {
        return Err("User declined to run the command".into());
    }

    Ok(response)
}

#[cfg(test)]
mod test {
    use core::{gpt_error::GptError, gpt_response::{Choice, GptResponse, Message}};

    use super::*;

    struct MockGptCall {
        response: String
    }

    impl GptCall for MockGptCall {
        fn call_gpt(&self, _: &str) -> Result<core::gpt_response::GptResponse, GptError> {
            Ok(GptResponse {
                choices: vec![Choice {
                    message: Message {
                        content: self.response.clone(),
                        role: "user".into()
                    }
                }]
            })
        }
    }

    #[test]
    fn test_get_gpt_result() {
        let mock_gpt = MockGptCall {
            response: "echo 'Hello World'".into()
        };

        let mut stdout = Vec::new();
        let mut stdin = std::io::Cursor::new("y\n");

        let result = get_gpt_result(mock_gpt, "echo 'Hello World'".into(), &mut stdout, &mut stdin);

        assert_eq!(result, Ok("echo 'Hello World'".into()));
    }

    #[test]
    fn test_get_gpt_result_decline() {
        let mock_gpt = MockGptCall {
            response: "echo 'Hello World'".into()
        };

        let mut stdout = Vec::new();
        let mut stdin = std::io::Cursor::new("n\n");

        let result = get_gpt_result(mock_gpt, "echo 'Hello World'".into(), &mut stdout, &mut stdin);

        assert_eq!(result, Err("User declined to run the command".into()));
    }
}