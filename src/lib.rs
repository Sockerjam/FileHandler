use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::{self, Write};

pub struct User {
    pub name: String,
    pub file_name: String,
    pub read: bool,
}

pub fn prompt(message: &str) -> String {
    println!("{message}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn open(file_path: &String) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(file_path.as_str())
}

pub fn write_to_file(file: &mut File, user_name: &String) -> io::Result<()> {
    let has_data = { file.metadata()?.len() != 0 };

    if has_data {
        file.seek(SeekFrom::End(0))?;
        file.write_all(b", ")?;
    }

    file.write_all(user_name.as_bytes())
}

pub fn confirm_action(input: &String) -> bool {
    match input.as_str() {
        "yes" => return true,
        "no" => return false,
        _ => return false,
    }
}

pub fn read_content_from(file: &String) -> Result<String, io::Error> {
    std::fs::read_to_string(file)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_confirm_action_returns_true() {
        let result = confirm_action(&String::from("yes"));
        assert!(result);
    }

    #[test]
    fn test_confirm_action_returns_false() {
        let result = confirm_action(&String::from("no"));
        assert!(!result);
    }
}
