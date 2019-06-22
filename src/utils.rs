use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::prelude::*;
use std::fs::File;

#[derive(PartialOrd, Ord, Debug)]
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
