use crate::meta_models::{Code, Command};

const SEPARATORS: &[char] = &[' ', '\t', '\n', '\r'];
const DELIMITERS: &[char] = &['\n', '\0'];

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
    DELIMITERS.contains(&c)
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

pub fn get_type_name<T>() -> &'static str {
    let full = std::any::type_name::<T>();
    full.rsplit("::").next().unwrap()
}

pub fn some_vec<T>(vec: Vec<T>) -> Option<Vec<T>> {
    if vec.is_empty() { None } else { Some(vec) }
}
