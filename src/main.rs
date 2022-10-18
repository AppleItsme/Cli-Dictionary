use std::{env, fs::{OpenOptions, File}, io::{Seek, Write, Read, stdin, stdout}, io::SeekFrom};

use rand::Rng;
//use regex::Regex;

fn main() {
    const NEW_LINE: u8 = '\n' as u8;
    const SPACE: u8 = ' ' as u8;

    println!("Personalised Dictionary\n");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./dictionary.txt")
        .unwrap();
    
    let mut file_buff: Vec<u8> = Vec::new(); 
    file.seek(SeekFrom::Start(file_buff.len() as u64)).unwrap();
    file.read_to_end(&mut file_buff).unwrap(); 
    
    file_buff.pop();


    loop {    
        println!("[r: revise; a: add; e: edit; q: quit]");
        let mut mode = String::new();
        let _ = stdin().read_line(&mut mode);
        mode.pop();
    
        match mode.as_str() {
            "r" => revision_mode(&file_buff),
            "a" => add_mode(&mut file),
            "e" => edit_mode(&file_buff, &file),
            "q" => return,
            _ => println!("unrecognised command")
        }
    }
    
}

fn edit_mode(buf: &Vec<u8>, file: &File) {
    println!("Searching in english? [y/n]");
    let mut input = String::new();
    while !(matches!(input.as_str(), "y\n" | "n\n")) {
        stdin().read_line(&mut input).unwrap();   
    }
    let english_search = input == "y\n";
    input.clear();
    print!("search: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();


    let file_str: String = String::from_utf8(buf.to_owned())
        .unwrap();
    let word_pairs: Vec<&str> = file_str.split("\n")
        .collect();

    input.pop();

    let mut special_lines: Vec<usize> = Vec::new();

    for (i, val) in word_pairs.iter().enumerate() {
        let pair: Vec<&str> = val.split("=").collect();
        let preferred_one = pair[english_search as usize];
    
        if preferred_one.find(&input) != None {
            println!("Entry {}) {}", i+1, preferred_one);
            special_lines.push(i);
        }
    }
    println!("Select the desired entry [c: cancel]");
    loop {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        input.pop();
        if input.as_str() == "c" {
            return;
        } else if !input.chars().all(char::is_numeric) {
            println!("Please just give me a NUMBER");
            continue;
        }
        break;
    }
}

fn add_mode(file: &mut File) {
    let mut word_pair = String::new();

    println!("Type the word to translate:");
    stdin().read_line(&mut word_pair).unwrap();
    word_pair = word_pair.replace("\n", "");
    word_pair.push('=');
    println!("Now the english version:");
    stdin().read_line(&mut word_pair).unwrap();
    
    let buf = word_pair.into_bytes();
    file.write_all(&buf).unwrap();
}

fn revision_mode(file_buffer: &Vec<u8>) {
    let text = String::from_utf8(file_buffer.to_owned()).unwrap();
    let lines: Vec<&str> = text
        .as_str()
        .split('\n')
        .collect();
    let chosen_pair: Vec<&str> = lines[rand::thread_rng().gen_range(0..lines.len())]
        .split("=")
        .collect();
    
    println!("Translate: {}; [s to show the answer]", chosen_pair[0]);
    let mut submitted_answer = String::new();
    while submitted_answer != "s" && submitted_answer != chosen_pair[1] {
        submitted_answer.clear();
        let _b1 = stdin().read_line(&mut submitted_answer);
        submitted_answer = submitted_answer.replace("\n", "");
    }
    if submitted_answer == chosen_pair[1] {
        println!("Good job!");
        return;
    }
    println!("The answer is: {}", chosen_pair[1]);
}
