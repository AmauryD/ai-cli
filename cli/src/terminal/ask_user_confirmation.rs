use std::{io::Write, io::BufRead};

pub fn ask_user_confirmation<T: Write, R: BufRead>(message: String, mut  stdout: T, mut stdin: R) -> bool {
    let mut input = String::new();
    writeln!(stdout, "{} (y/n): ", message).unwrap();
    stdin
        .read_line(&mut input)
        .map_or(false, |_| input.to_lowercase().starts_with("y"))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_ask_user_confirmation_yes() {
        let mut stdin = Cursor::new("y\n");
        let mut stdout = Vec::new();
        assert_eq!(
            ask_user_confirmation("Do you want to continue?".to_string(), &mut stdout, &mut stdin),
            true
        );
    }

    #[test]
    fn test_ask_user_confirmation_no() {
        let mut stdin = Cursor::new("n\n");
        let mut stdout = Vec::new();
        assert_eq!(
            ask_user_confirmation("Do you want to continue?".to_string(), &mut stdout, &mut stdin),
            false
        );
    }

    #[test]
    fn test_must_write_message_to_stdout() {
        let mut stdin = Cursor::new("n\n");
        let mut stdout = Vec::new();
        ask_user_confirmation("Do you want to continue?".to_string(), &mut stdout, &mut stdin);
        assert_eq!(String::from_utf8(stdout).unwrap(), "Do you want to continue? (y/n): \n");
    }
}
