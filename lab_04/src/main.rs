use std::{fs, io};

use crate::CustomError::NotALetter;

fn main() {
    match do_stuff() {
        Ok(()) => (),
        Err(x) => println!("{x}"),
    }
}

fn do_stuff() -> Result<(), io::Error> {
    //am dat call initial la toate functiile din dostuff si am aflat acum la final ca nu pot folosii ? direct in main in cazul asta fiindca nu returneaza io error
    let s = fs::read_to_string("src/inputfile.txt")?;
    print_most_bytes_line(&s);
    print_most_chars_line(&s);
    print_rot_file_x(&s, 13);
    println!(
        "{x}",
        x = abbreviate("Am fost la dl Matei pt că m-a invitat cu o zi înainte")
    );

    match print_hosts("src/hosts.txt") {
        Ok(_) => {}
        Err(e) => println!("Could not find src/hosts.txt , {e}"),
    }

    Ok(())
}

fn print_most_bytes_line(textfile: &str) {
    let mut max_bytes = 0;

    let mut longest_line: &str = "";

    for line in textfile.lines() {
        let current_bytes = line.len();

        if current_bytes > max_bytes {
            max_bytes = current_bytes;
            longest_line = line;
        }
    }

    if !longest_line.is_empty() {
        println!("{longest_line}");
    }
}

fn print_most_chars_line(textfile: &str) {
    let mut max_char_count = 0;

    let mut longest_line: &str = "";

    for line in textfile.lines() {
        let current_char_count = line.chars().count();

        if current_char_count > max_char_count {
            max_char_count = current_char_count;
            longest_line = line;
        }
    }

    if !longest_line.is_empty() {
        println!("{longest_line}");
    }
}

fn rot_char_x(mut ch: char, offset: u32) -> Result<char, CustomError> {
    let normalized_offset: u8 = (offset % 26) as u8;

    if ch.is_ascii_lowercase() {
        ch = (b'a' + (ch as u8 - b'a' + normalized_offset) % 26) as char;
        return Ok(ch);
    }

    if ch.is_ascii_uppercase() {
        ch = (b'A' + (ch as u8 - b'A' + normalized_offset) % 26) as char;
        return Ok(ch);
    }

    Err(NotALetter)
}

fn print_rot_file_x(textfile: &str, offset: u32) {
    let mut result: String = String::new();
    for ch in textfile.chars() {
        match rot_char_x(ch, offset) {
            Ok(x) => result.push(x),
            Err(_) => result.push(ch),
        }
    }
    println!("{result}");
}

fn abbreviate(textfile: &str) -> String {
    let abbreviations = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let mut result = String::new();

    for word in textfile.split_whitespace() {
        let mut replacement = word;

        for (abbr, full) in &abbreviations {
            if word == *abbr {
                replacement = full;
                break;
            }
        }

        if !result.is_empty() {
            result.push(' ');
        }

        result.push_str(replacement);
    }

    result
}

fn print_hosts(file_path: &str) -> io::Result<()> {
    let textfile = fs::read_to_string(file_path)?;

    for line in textfile.lines() {
        if line.starts_with('#') {
            continue;
        }

        let mut col1 = "";
        let mut col2 = "";
        let mut count = 0;

        for word in line.split_whitespace() {
            if count == 0 {
                col1 = word;
            } else if count == 1 {
                col2 = word;
            }
            count += 1;
        }

        if count >= 2 {
            println!("{} => {}", col2, col1);
        }
    }

    Ok(())
}

pub enum CustomError {
    NotALetter,
}
