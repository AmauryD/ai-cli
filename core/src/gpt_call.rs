use ureq::post;

use crate::{configuration::Configuration, context::Context, gpt_error::GptError, gpt_response::GptResponse};

pub trait GptCall {
    fn call_gpt(&self, ask: &str) -> Result<GptResponse, GptError>;
}

pub struct GptCallImpl {
    config: Configuration,
    context: Context,
}

impl GptCallImpl {
    pub fn new(config: Configuration, context: Context) -> GptCallImpl {
        GptCallImpl { config, context }
    }

    pub fn get_body(&self, ask: &str) -> serde_json::Value {
        serde_json::json!({
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
                    "content": format!("My file tree is : {}", self.context.file_tree)
                },
                {
                    "role": "user",
                    "content": format!("My environment is : {}", self.context.env_vars)
                },
                {
                    "role": "user",
                    "content": format!("My last commands are : {}", self.context.last_shell_commands)
                },
                {
                    "role": "user",
                    "content": format!("How to {}", ask)
                }
            ],
            "stream": false
        })
    }
}

impl GptCall for GptCallImpl {
    fn call_gpt(&self, ask: &str) -> Result<GptResponse, GptError> {
        let response: GptResponse = post(self.config.gpt_url.as_str())
            .set("Content-Type", "application/json")
            .send_json(self.get_body(ask)).map_err(|_| GptError::FailedToCallGpt)?
            .into_json().map_err(|_| GptError::FailedToCallGpt)?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{Method::POST, MockServer};

    #[test]
    fn test_get_body() {
        use super::GptCallImpl;
        use crate::configuration::Configuration;
        use crate::context::Context;

        let config = Configuration {
            gpt_url: "gpt_url".to_string(),
            model: "gpt-3.5-turbo".to_string(),
        };
        let context = Context {
            env_vars: "env_vars".to_string(),
            cwd: "cwd".to_string(),
            available_commands: "available_commands".to_string(),
            last_shell_commands: "".into(),
            file_tree: "file_tree".to_string(),
        };
        let gpt_call = GptCallImpl::new(config, context);

        let body = gpt_call.get_body("find the best command");
        assert_eq!(body["model"], "gpt-3.5-turbo");
        assert_eq!(body["messages"][0]["role"], "system");
        assert_eq!(body["messages"][0]["content"], "You're a command line assistant. You help find the best terminal command. Don't ask user questions. Answer only with the best command.");
        assert_eq!(
            body["messages"][1]["content"],
            "The available terminal commands are : available_commands"
        );
        assert_eq!(
            body["messages"][2]["content"],
            "My current directory is : cwd"
        );
        assert_eq!(body["messages"][3]["content"], "My file tree is : file_tree");
        assert_eq!(body["messages"][4]["content"], "My environment is : env_vars");
        assert_eq!(body["messages"][5]["content"], "My last commands are : ");
    }

    #[test]
    fn test_call_gpt() {
        use super::GptCall;
        use super::GptCallImpl;
        use crate::configuration::Configuration;
        use crate::context::Context;


        let context = Context {
            env_vars: "env_vars".to_string(),
            cwd: "cwd".to_string(),
            last_shell_commands: "".into(),
            available_commands: "available_commands".to_string(),
            file_tree: "file_tree".to_string(),
        };

        let server = setup_mock_server();

        let config = Configuration {
            gpt_url:  server.url("/"),
            model: "gpt-3.5-turbo".to_string(),
        };

        let gpt_call = GptCallImpl::new(config, context);


        let response = gpt_call.call_gpt("find the best command");
        assert!(response.unwrap().choices.first().unwrap().message.content == "ls -l");
    }

    fn setup_mock_server() -> MockServer {
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(POST).path("/");
            then.header("content-type", "application/json").body(
                r#"{
                        "choices": [
                            {
                                "message": {
                                    "content": "ls -l",
                                    "role": "assistant"
                                }
                            }
                        ]
                    }"#,
            );
        });
        server
    }
}
