use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::prelude::*;
use std::fs::File;

#[derive(PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Pos {
    line: u64,
    col: u64
}

impl Pos {
    pub fn new(line: u64, col: u64) -> Self {
        Self { line, col  }
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn col(&self) -> u64 {
        self.col
    }

    pub fn get(&self) -> (u64, u64) {
        (self.line, self.col)
    }

    fn to_char_index(&self) -> (u64, u64) {
        (self.line-1, self.col-1)
    }

}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.line == other.line
    }
}

impl<'a> PartialEq<&'a Pos> for Pos {
    fn eq(&self, other: &&Self) -> bool {
        self.col == other.col && self.line == other.line
    }
}


impl Eq for Pos { }

impl<'a> PartialOrd<&'a Pos> for Pos {
    fn partial_cmp(&self, other: &&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone)]
pub enum BPairs {
    Brack,
    Paren,
    Curly
}

impl BPairs{

    pub fn to_string_pair(&self) -> (String, String) {
        match &self {
            BPairs::Brack   =>  (String::from(r"\["), String::from(r"\]")),
            BPairs::Paren   =>  (String::from("("), String::from(")")),
            BPairs::Curly   =>  (String::from("{"), String::from("}")),
        }
    }

    pub fn to_simple_string_open(&self) -> String {
        match &self {
            BPairs::Brack   =>  String::from("["),
            BPairs::Paren   =>  String::from("("),
            BPairs::Curly   =>  String::from("{")
        }
    }

    pub fn to_simple_string_close(&self) -> String {
        match &self {
            BPairs::Brack   =>  String::from("]"),
            BPairs::Paren   =>  String::from(")"),
            BPairs::Curly   =>  String::from("}")
        }
    }


    pub fn array() -> [BPairs;3] {
        [BPairs::Brack, BPairs::Paren, BPairs::Curly]
    }
}


#[derive(Debug)]
pub struct Log {
    fd: File,
}

impl Log {

    pub fn new(name: &str) -> Log {
        Log { fd: File::create(name).unwrap() }
    }

    pub fn log(&mut self, string: &str) {
        self.fd.write(string.as_bytes()).unwrap();
    }

}

pub trait Logger {
    fn log(&mut self, string: &str) { }
    fn log_err<T: Debug>(&mut self, string: &str, err: T) { }
}
