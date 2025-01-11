use crate::piece::piece::PieceUnicode;
use crate::piece::piece::{Piece, Player};
use std::collections::HashMap;
use std::fmt::Error;
use std::fmt::Formatter;
use std::io;

use std::fmt::Display;

pub struct BoardGeneration {
    pub _board: [[Option<Piece>; 8]; 8],
    pub _current: Player,
}

impl BoardGeneration {
    pub fn new() -> Self {
        Self {
            _board: [[None; 8]; 8],
            _current: Player::Black,
        }
    }

    pub fn normal_chessboard(&mut self) {
        let white_pawn = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_pawn(),
        );
        let white_knight = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_knight(),
        );
        let white_bishop = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_bishop(),
        );
        let white_rook = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_rook(),
        );
        let white_king = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_king(),
        );
        let white_queen = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_white_queen(),
        );
        let black_pawn = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_pawn(),
        );
        let black_knight = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_knight(),
        );
        let black_bishop = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_bishop(),
        );
        let black_rook = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_rook(),
        );
        let black_queen = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_queen(),
        );
        let black_king = Piece::new(
            1,
            1,
            1,
            "running",
            "running",
            1,
            PieceUnicode::get_black_king(),
        );
        self._board = [
            [
                Some(black_rook),
                Some(black_knight),
                Some(black_bishop),
                Some(black_queen),
                Some(black_king),
                Some(black_bishop),
                Some(black_knight),
                Some(black_rook),
            ],
            [Some(black_pawn); 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [Some(white_pawn); 8],
            [
                Some(white_rook),
                Some(white_knight),
                Some(white_bishop),
                Some(white_king),
                Some(white_queen),
                Some(white_bishop),
                Some(white_knight),
                Some(white_rook),
            ],
        ]
    }
    pub fn game_cycle(&mut self){
        let mut running = true;
        
        while running {
            println!("{}", self);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_string();
            match Some(input) {
                Some(value) => {
                    match self.select_piece(value) {
                        Some(val) => println!("The piece you selected is {}\n", val),
                        None => println!("You selected an empty space")
                    }
                }
                None => running = false
            }
        }
        self.close_program();
    }

    pub fn close_program(&self) {
        println!("The Chess program have closed");
    }

    fn get_hashmap(&self) -> HashMap<char, i32>{
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let alpha: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let mut map = HashMap::new();
        for i in 0..numbers.len() {
            let _ = &map.insert(alpha[i], numbers[i]);
        }
        return map;
    }

    fn select_piece(&mut self, value: String) -> Option<Piece>{
        if value.len() == 2 {
            let sub_strings: Vec<char> = value.chars().collect();
            let a: usize = sub_strings[0].to_digit(10).unwrap_or_default() as usize;
            let b: usize = self.get_hashmap().get(&sub_strings[1]).cloned().unwrap() as usize;
            return self._board[a-1][b-1];
        }
        return None;
    }
}

impl Display for BoardGeneration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        println!("   A B C D E F G H ");
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        for (rank, row) in self._board.iter().enumerate() {
            print!("{} ", numbers[rank]);
            for cell in row {
                match cell {
                    Some(piece) => write!(f, "|{}", piece)?,
                    None => write!(f, "| ")?,
                }
            }
            print!("|\n");
        }
        Ok(())
    }
}


// King: ♔ (U+2654)
// Queen: ♕ (U+2655)
// Rook: ♖ (U+2656)
// Bishop: ♗ (U+2657)
// Knight: ♘ (U+2658)
// Pawn: ♙ (U+2659)
// Black Pieces:

// King: ♚ (U+265A)
// Queen: ♛ (U+265B)
// Rook: ♜ (U+265C)
// Bishop: ♝ (U+265D)
// Knight: ♞ (U+265E)
// Pawn: ♟ (U+265F)
