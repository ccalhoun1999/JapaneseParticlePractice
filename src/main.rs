use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "sentences.utf";
    let file = match File::open(&file_path) {
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    let mut line_iter = reader.lines().map(|line| line.unwrap());
    
    get_user_input("test");

    while let (Some(raw_line), Some(annotated_line)) = (line_iter.next(), line_iter.next()) {
        println!("Line 1: {}", remove_prefix(&raw_line));
        println!("Line 2: {}", clean_string(&remove_prefix(&annotated_line)));
        return;
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim_end().to_string()
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            String::new()
        }
    }
}

fn remove_prefix(text: &str) -> String {
    if text.len() >= 3 {
        String::from(&text[3..])
    } else {
        String::from("")
    }
}

fn clean_string(text: &str) -> String {
    let enclosures = Regex::new(r"\([^()]*\)|\{[^{}]*\}|\[[^\[\]]*\]").unwrap();
    let mut parsing = String::from(enclosures.replace_all(text, "").to_string());
    let particles = Regex::new(r"\s(は|の|で|と|が)\s").unwrap();
    parsing = String::from(particles.replace_all(&parsing, "_"));
    let spaces = Regex::new(r"\s").unwrap();
    parsing = String::from(spaces.replace(&parsing, ""));
    parsing
}