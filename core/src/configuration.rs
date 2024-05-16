
pub struct Configuration<'a> {
    pub gpt_url: &'a str,
    pub model: &'a str,
}   

impl Configuration<'_> {
    pub fn new<'a>(gpt_url: &'a str, model: &'a str) -> Configuration<'a> {
        Configuration {
            gpt_url: gpt_url,
            model: model,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_configuration() {
        let config = Configuration::new("https://api.openai.com/v1/engines/davinci/completions", "davinci");
        assert_eq!(config.gpt_url, "https://api.openai.com/v1/engines/davinci/completions");
    }
}
