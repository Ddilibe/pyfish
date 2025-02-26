// Built-in Import
// use std::io;
// Crates
use crate::engine;
// use crate::piece;
// Custom Imports
use engine::boardgeneration::BoardGeneration;
// use piece::piece::Player;



pub struct NewGame<'a> {
    pub board: &'a mut BoardGeneration,
}

impl<'a> NewGame<'a> {
    pub fn new(board: &'a mut BoardGeneration) -> Self {
        Self { 
            board: board
         }
    }
    pub fn start(&mut self) {
        println!("**************************");
        println!("**   Starting New Game  **");
        println!("**************************\n\n");
        self.board.normal_chessboard();
        self.board.game_cycle();
    }
}