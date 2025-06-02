use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::io::{ErrorKind, Read};

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

pub fn open(filePath: &String) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(filePath.as_str())
}

pub fn write_to_file(file: &mut File, user_name: &String) -> Result<(), io::Error> {
    let has_users = match file_has_multiple_users(file) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    if has_users {
        let mut user_name_edit = String::from(", ");
        user_name_edit.push_str(user_name.as_str());
        let _ = file.write(user_name_edit.as_bytes())?;
        return Ok(());
    } else {
        let _ = file.write(user_name.as_bytes())?;
        return Ok(());
    }
}

fn file_has_multiple_users(file: &mut File) -> io::Result<bool> {
    let mut buffer = Vec::new();
    let bytes = file.read_to_end(&mut buffer)?;
    Ok(bytes != 0)
}

pub fn should_read_content(input: &String) -> bool {
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
    fn test_should_read_content_returns_true() {
        let result = should_read_content(&String::from("yes"));
        assert!(result);
    }

    #[test]
    fn test_should_read_content_returns_false() {
        let result = should_read_content(&String::from("no"));
        assert!(!result);
    }
}
