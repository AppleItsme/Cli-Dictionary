use std::{fs::{OpenOptions, File}, io::{Seek, Write, Read, stdin, stdout}, io::SeekFrom};


use rand::Rng;
//use regex::Regex;

fn main() {
    println!("Personalised Dictionary\n");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./dictionary.txt")
        .unwrap();
    
    let mut file_buff: Vec<u8> = Vec::new(); 
    
    file_buff.pop();


    loop {
        file_buff.clear();
        file.seek(SeekFrom::Start(file_buff.len() as u64)).unwrap();
        file.read_to_end(&mut file_buff).unwrap();
        
        println!("[r: revise; a: add; e: edit; q: quit]");
        let mut mode = String::new();
        let _ = stdin().read_line(&mut mode);
        mode = clean_string(mode);
    
        match mode.as_str() {
            "r" => revision_mode(&file_buff),
            "a" => add_mode(&mut file),
            "e" => edit_mode(&file_buff, &mut file),
            "q" => return,
            _ => println!("unrecognised command")
        }
    }
    
}





fn clean_string(input: String) -> String {
    input
        .replace(" ", "")
        .replace("\n", "")
}

fn remove_whitespace_suffix(input: &mut String) {
    let has_new_line = input.ends_with('\n');
    if has_new_line {
        input.pop();
    }
    while input.ends_with(' ') {
        input.pop();
    }
    if has_new_line {
        input.push('\n');
    }
}

fn edit_mode(buf: &Vec<u8>, file: &mut File) {
    println!("Searching in english? [y/n]");
    let mut input = String::new();
    while !(matches!(input.as_str(), "y" | "n")) {
        stdin().read_line(&mut input).unwrap();   
        input = clean_string(input);
    }
    let english_search = input == "y";
    input.clear();
    print!("search: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input.pop();
    remove_whitespace_suffix(&mut input);

    let file_str: String = String::from_utf8(buf.to_owned())
        .unwrap();
    let mut word_pairs: Vec<Vec<&str>> = file_str
        .split("\n")
        .map(|x| x.split('=').collect())
        .collect();
    word_pairs.pop();


    let mut special_lines: Vec<usize> = Vec::new();

    let mut j = 0;

    for (i, val) in word_pairs.iter().enumerate() {
        let preferred_one = val[english_search as usize];
    
        if preferred_one.find(&input) != None {
            j += 1;
            println!("{}) {}", j, preferred_one);
            special_lines.push(i);
        }
    }
    if special_lines.is_empty() {
        println!("Couldn't find anything with that, sorry");
        return;
    }
    println!("Select the desired entry [c: cancel]");
    let mut chosen_i;
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
        chosen_i = input.parse::<usize>().unwrap();
        if chosen_i > j {
            println!("please select an appropriate number");
            continue;
        }
        break;
    }
    let old_word_pair = format!("{}={}", 
                                word_pairs[special_lines[chosen_i-1]][0],
                                word_pairs[special_lines[chosen_i-1]][1]); 


    let mut updated_word_pair = String::new();
    print!("Type the updated word (It was {}):", word_pairs[special_lines[chosen_i-1]][0]);
    stdout().flush().unwrap();
    stdin().read_line(&mut updated_word_pair).unwrap();
    remove_whitespace_suffix(&mut updated_word_pair);

    print!("Type the updated translation (It was {}):", word_pairs[special_lines[chosen_i-1]][1]);
    stdout().flush().unwrap();
    updated_word_pair.push('=');
    stdin().read_line(&mut updated_word_pair).unwrap();  

    remove_whitespace_suffix(&mut updated_word_pair);

    let tmp: Vec<&[u8]> = file_str
        .split(old_word_pair.as_str())
        .map(|x| x.as_bytes())
        .collect();

    updated_word_pair = updated_word_pair.replace("\n", "");

    file.set_len(tmp[0].len() as u64).unwrap();
    file.write_all(updated_word_pair.as_bytes()).unwrap();
    file.write_all(tmp[1]).unwrap();
}

fn add_mode(file: &mut File) {
    let mut word_pair = String::new();

    println!("Type the word to translate:");
    stdin().read_line(&mut word_pair).unwrap();
    word_pair = word_pair.replace("\n", "");

    remove_whitespace_suffix(&mut word_pair);

    word_pair.push('=');
    println!("Now the english version:");
    stdin().read_line(&mut word_pair).unwrap();
   
    remove_whitespace_suffix(&mut word_pair);
    

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
        remove_whitespace_suffix(&mut submitted_answer);
    }
    if submitted_answer == chosen_pair[1] {
        println!("Good job!");
        return;
    }
    println!("The answer is: {}", chosen_pair[1]);
}
