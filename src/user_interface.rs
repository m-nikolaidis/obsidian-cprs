use std::io::{self, Write};

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> UserInterface {
        UserInterface
    }

    pub fn prompt_user(&self, message: &str) -> String {
        print!("{} ", message);
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        user_input.trim().to_string()
    }

    pub fn display_message(&self, message: &str) {
        println!("{}", message);
    }
}
