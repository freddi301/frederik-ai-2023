use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::Read;

pub fn read_file_to_string(file_path: &str) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

pub fn clean_data(string: &String) -> Vec<char> {
    let is_alpha = Regex::new("[a-zA-Z .,]").unwrap();
    let cleaned = string
        .chars()
        .map(|character| character.to_ascii_lowercase())
        .filter(|character| is_alpha.is_match(&character.to_string()))
        .collect();
    cleaned
}

fn main() {}
