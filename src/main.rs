use std::io::{Write, stdin, stdout};
use cli_dictionary::{ssf_format::SsfInstance, dictionary_profile::startup, clean_string, remove_whitespace_suffix};
use rand::{Rng, thread_rng};

fn main() {

    println!("Personalised Dictionary\n");

    let path = startup(); 
    let mut dictionary: SsfInstance;
    match path {
        Some(v) => dictionary = SsfInstance::new(&v),
        None => return,
    }
    

    loop {
        println!("[r: revise; a: add; e: edit; q: quit]");
        let mut mode = String::new();
        let _ = stdin().read_line(&mut mode);
        mode = clean_string(mode);
    
        match mode.as_str() {
            "r" => revision_mode(&mut dictionary),
            "a" => add_mode(&mut dictionary),
            "e" => edit_mode(&mut dictionary),
            "q" => return,
            _ => println!("unrecognised command")
        }
    }
    
}


fn edit_mode(ssf_instance: &mut SsfInstance) {
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
    input = remove_whitespace_suffix(input);

    let word_pairs = ssf_instance.parse();
    let mut special_lines: Vec<usize> = Vec::new();
    let mut j = 0;

    for (i, val) in word_pairs.iter().enumerate() {
        let preferred_one = &val[english_search as usize];
    
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
    let old_word_pair = word_pairs[special_lines[chosen_i-1]].to_owned();


    let mut updated_word_pair: Vec<String> = vec![String::new(), String::new()];
    print!("Type the updated word (It was {}):", old_word_pair[0]);
    stdout().flush().unwrap();
    stdin().read_line(&mut updated_word_pair[0]).unwrap();

    print!("Type the updated translation (It was {}):", old_word_pair[1]);
    stdout().flush().unwrap();
    stdin().read_line(&mut updated_word_pair[1]).unwrap();  

    updated_word_pair = updated_word_pair.iter()
        .map(|x| remove_whitespace_suffix(x.to_owned()))
        .collect();
    updated_word_pair[1].push('\n');

    ssf_instance.replace_entry(old_word_pair, updated_word_pair);
}

fn add_mode(ssf_instance: &mut SsfInstance) {
    let mut word_pair: Vec<String> = vec![String::new(), String::new()];

    println!("Type the word to translate:");
    stdin().read_line(&mut word_pair[0]).unwrap();

    println!("Now the english version:");
    stdin().read_line(&mut word_pair[1]).unwrap();

    word_pair = word_pair.iter()
        .map(|x| remove_whitespace_suffix(x.to_owned()))
        .collect();
    word_pair[1].push('\n');

    ssf_instance.new_entry(word_pair);
}


fn revision_mode(ssf_instance: &mut SsfInstance) {
    let entries = ssf_instance.parse();
    
    if entries.len() == 0 {
        println!("No words are found! Let's add one!");
        add_mode(ssf_instance);
        return;
    }

    let chosen_pair = &entries[thread_rng().gen_range(0..entries.len())];

    println!("Translate: {}; [s to show the answer]", chosen_pair[0]);
    let mut submitted_answer = String::new();
    while submitted_answer != "s" && submitted_answer != chosen_pair[1] {
        submitted_answer.clear();
        let _b1 = stdin().read_line(&mut submitted_answer);
        submitted_answer = remove_whitespace_suffix(submitted_answer);
    }
    if submitted_answer == chosen_pair[1] {
        println!("Good job!");
        return;
    }
    println!("The answer is: {}", chosen_pair[1]);
}
