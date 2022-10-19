use std::{fs::{self, create_dir_all, remove_file}, env::consts::OS, io::{stdin, stdout, Write}, process::Command};

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
        println!("{}) {}", i+1, val);
        }
        println!("[a: add a new dictionary; 0-{}: pick the dictionary; d: delete; q: quit]", files.len());
        
        let mut option = String::new();
        loop {
        option.clear();
        stdin().read_line(&mut option).unwrap();
        option = clean_string(option);
        if &option == "q" {
            return None
        } else if &option == "d" {
            println!("Choose which file to delete");
            stdout().flush().unwrap();
            let mut index = String::new();
            loop {
                stdin().read_line(&mut index).unwrap();
                index = clean_string(index);
                if let Ok(n) = index.parse::<usize>() {
                    if cfg!(windows) {
                        Command::new("del")
                            .arg(format!("\"{}{}{}\"", path, slash, files[n-1]))
                            .spawn()
                            .unwrap();
                    } else {
                        Command::new("rm")
                            .arg(format!("{}{}{}", path, slash, files[n-1]))
                            .spawn()
                            .unwrap();
                    }
                    return None;
                }
                println!("Give an appropriate number");
            }
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
            let index = option.parse::<usize>().unwrap()-1;
            if index > files.len() {
                println!("Enter a number within the existing bounds please");
                continue;
            }
            return Some(files[index].to_owned());
        }
        println!("Unrecognised option");
    }
}
