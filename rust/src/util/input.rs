use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub struct Input {
    file: File,
    split: Option<String>,
}

pub fn get_input(path: &str) -> self::Input {
    let filepath = Path::new(path);
    let file = File::open(filepath).expect("File not found");

    Input {
        file: file,
        split: None,
    }
}

impl Input {
    pub fn by(&mut self, split: String) -> &mut Self {
        self.split = Some(String::from(split));
        self
    }

    pub fn as_chars(&mut self) -> Vec<char> {
        let buf = self.as_bytes();
        String::from_utf8(buf)
            .expect("from_utf8 failed")
            .chars()
            .collect()
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        // read the whole file
        &self.file.read_to_end(&mut buffer);
        buffer
    }

    pub fn as_ints(&self) -> Vec<i32> {
        let mut nums: Vec<i32> = Vec::new();
        let f = BufReader::new(&self.file);
        for line in f.lines() {
            nums.push(line.unwrap().parse().unwrap());
        }

        nums
    }

    pub fn as_strings(&self) -> Vec<String> {
        let mut strs: Vec<String> = Vec::new();
        let f = BufReader::new(&self.file);
        for line in f.lines() {
            strs.push(line.unwrap());
        }
        strs
    }
}
