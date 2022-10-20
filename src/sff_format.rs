use std::{fs::{File, OpenOptions}, io::{Seek, SeekFrom, Read, Write}};

use crate::dictionary_profile::NEWLINE;

pub struct SsfInstance {
    byte_buf: Vec<u8>,
    string_buf: String,
    file: File,
    path: String
}

//Seg'S Format

impl SsfInstance {
    pub fn new(path: &str) -> Self {
        let mut file: File;
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path) {
                Ok(v) => file = v,
                Err(e) => panic!("Error while opening the file at {}: {}", path, e)
        }

        let mut byte_buf: Vec<u8> = Vec::new();
        let mut string_buf = String::new();
        if let Err(e) = file.seek(SeekFrom::Start(0)) {
            panic!("Error while seeking: {}", e);
        }
        if let Err(e) = file.read_to_end(&mut byte_buf) {
            panic!("Could not read the file: {}", e);
        }
        match String::from_utf8(byte_buf.to_owned()) {
            Ok(v) => string_buf = v,
            Err(e) => panic!("Wow the file is corrupted.\n{}", e)
        }
        Self {
            byte_buf,
            string_buf,
            file,
            path: String::from(path)
        }
    }

    pub fn parse(&self) -> Vec<Vec<String>> {
    //Parse SSF into vector of vectors. Second vector always has the length of 2 
        let mut parsed: Vec<Vec<String>> = self.string_buf
            .split(NEWLINE)
            .map(|x| x
                 .split('=')
                 .map(|y| String::from(y))
                 .collect()
                 )
            .collect();
        parsed.pop();
        parsed
    }
    pub fn new_entry(&mut self, entry: Vec<String>) {
        let entry_line = format!("{}={}", entry[0], entry[1]);
        if let Err(e) = self.file.write_all(entry_line.as_bytes()) {
            panic!("Cannot write to a file: {}", e);
        }
        *self = SsfInstance::new(&self.path)
    }

    pub fn replace_entry(&mut self, old: Vec<String>, new: Vec<String>) {
        let tmp: Vec<&[u8]> = self.string_buf
            .split(format!("{}={}", old[0], old[1]).as_str())
            .map(|x| x.as_bytes())
            .collect();
        self.file.set_len(tmp[0].len() as u64).unwrap();
        self.file.seek(SeekFrom::Start(tmp[0].len() as u64)).unwrap();
        self.file.write_all(format!("{}={}", new[0], new[1]).as_bytes()).unwrap();
        self.file.write_all(tmp[1]).unwrap();

        *self = SsfInstance::new(&self.path)
    }
}
