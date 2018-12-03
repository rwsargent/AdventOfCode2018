use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Input {
    file: File,
}

pub fn get_input(path: &str) -> self::Input {
    let filepath = Path::new(path);
    let file = File::open(filepath).expect("File not found");

    Input{ file : file}
}

impl Input {
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
