use std::{fs::{self, create_dir_all}, env::consts::OS, io::{stdin, stdout, Write}};

use home::home_dir;

use crate::clean_string;

pub fn startup() -> Option<String> {
    let slash = match OS {
        "windows" => r"\",
        _ => "/"
    };
    let path = &format!("{}{}{}", 
            home_dir().unwrap().display(),
            slash,
            ".cli_dict"
    );
    create_dir_all(path).unwrap();

    let path_with_slash: String = format!("{}{}", path, slash);
    let files: Vec<String> = fs::read_dir(path)
        .unwrap()
        .map(|x| format!("{}",x
                         .unwrap()
                         .path()
                         .display()
                         )
                .replace(&path_with_slash, "")
                )
        .filter(|x| x.ends_with(".ssf"))
        .collect();
    for (i, val) in files.iter().enumerate() {
        println!("{}) {}", i, val);
    }
    println!("[a: add a new dictionary; 0-{}: pick the dictionary; q: quit]", files.len());

    let mut option = String::new();
    loop {
        option.clear();
        stdin().read_line(&mut option).unwrap();
        option = clean_string(option);
        if &option == "q" {
            return None
        } else if files.len() == 0 || option.as_str() == "a" {
            if option.chars().all(char::is_numeric) {
                println!("Heya you didn't think you would were smart and found a bug there did'ya?");
            }
            let mut name = String::new();
            print!("Name for the dictionary (NO SPACES WILL BE REGISTERED):");
            stdout().flush().unwrap();
            while name.is_empty() {
                stdin().read_line(&mut name).unwrap();
                name = clean_string(name);
            }
            return Some(format!("{}{}{}.ssf", path, slash, name));
        } else if option.chars().all(char::is_numeric) {
            return Some(files[option.parse::<usize>().unwrap()].to_owned());
        }
        println!("Unrecognised option");
    }
}
