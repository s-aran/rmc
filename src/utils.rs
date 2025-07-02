use crate::meta_models::{Code, Command};

const SEPARATORS: &[char] = &[' ', '\t', '\n', '\r'];
const DELIMITER: char = '\n';

pub fn split(target: &str) -> Vec<&str> {
    target
        .split(SEPARATORS)
        .filter(|e| !e.trim().is_empty())
        .collect()
}

pub fn is_sep(c: char) -> bool {
    SEPARATORS.contains(&c)
}

pub fn is_n(c: char) -> bool {
    c == DELIMITER
}

pub trait ParseUtil {
    fn get_mml(&self) -> &String;

    fn get_code(&self) -> &Code;

    fn clone_code(&self) -> Code {
        self.get_code().clone()
    }


    fn current_line(&self) -> String {
        let mml = self.get_mml();
        let lines = mml.lines().collect::<Vec<&str>>();
        lines.get(self.get_code().lines).unwrap().to_string()
    }

    fn parse_command(&self, c: char) -> Command;
}
