use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum MyError {
    InvalidMoveError,
    IllegalMoveError,
    AmbigousMoveError,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: fmt::Formatter) {
        match self{
            MyError::InvalidMoveError => write!(f, "Move notation is not syntactically valid"),
            MyError::IllegalMoveError => write!(f, "The attempted move is illegal in the current position"),
            MyError::AmbigousMoveError => write!(f, "The attempted move is ambiguous in the current position"),
        }
    }
}