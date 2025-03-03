use crate::piece::piece::{Piece, Player, PieceUnicode};
use std::cmp::Ordering;
use std::fmt::{Error, Formatter, Display};
use std::collections::HashMap;
use std::process::Command;
use std::thread::sleep;
use std::{io, time};

pub struct BoardGeneration {
    pub _board: [[Option<Piece>; 8]; 8],
    pub _current: Player,
}

#[allow(unused_doc_comments)]
impl BoardGeneration {
    pub fn new() -> Self {
        Self {
            _board: [[None; 8]; 8],
            _current: Player::White,
        }
    }

    pub fn new_normal_chessboard(&mut self) {
        let mut white_pawn = Piece::new(
            1,
            1,
            0b1111111100000000,
            "white pawn",
            "P",
            "running",
            0x000000000000ff00,
            PieceUnicode::get_white_pawn(),
            Player::White,
        );
        let mut white_knight = Piece::new(
            1,
            1,
            0b1000010,
            "running",
            "N",
            "running",
            0x0000000000000042,
            PieceUnicode::get_white_knight(),
            Player::White,
        );
        let mut white_bishop = Piece::new(
            1,
            1,
            0b100100,
            "running",
            "B",
            "running",
            0x0000000000000024,
            PieceUnicode::get_white_bishop(),
            Player::White,
        );
        let mut white_rook = Piece::new(
            1,
            1,
            0b10000001,
            "running",
            "R",
            "running",
            0x0000000000000081,
            PieceUnicode::get_white_rook(),
            Player::White,
        );
        let mut white_king = Piece::new(
            1,
            1,
            0b1000,
            "running",
            "K",
            "running",
            0x0000000000000008,
            PieceUnicode::get_white_king(),
            Player::White,
        );
        let mut white_queen = Piece::new(
            1,
            1,
            0b0000000000000000000000000000000000000000000000000000000000010000,
            "running",
            "Q",
            "running",
            0x0000000000000010,
            PieceUnicode::get_white_queen(),
            Player::White,
        );
        let mut black_pawn = Piece::new(
            1,
            1,
            0b11111111000000000000000000000000000000000000000000000000,
            "running",
            "p",
            "running",
            0x00ff000000000000,
            PieceUnicode::get_black_pawn(),
            Player::Black,
        );
        let mut black_knight = Piece::new(
            1,
            1,
            0b100001000000000000000000000000000000000000000000000000000000000,
            "running",
            "n",
            "running",
            0x4200000000000000,
            PieceUnicode::get_black_knight(),
            Player::Black,
        );
        let mut black_bishop = Piece::new(
            1,
            1,
            0b10010000000000000000000000000000000000000000000000000000000000,
            "running",
            "b",
            "running",
            0x2400000000000000,
            PieceUnicode::get_black_bishop(),
            Player::Black,
        );
        let mut black_rook = Piece::new(
            1,
            1,
            0b1000000100000000000000000000000000000000000000000000000000000000,
            "running",
            "r",
            "running",
            0x8100000000000000,
            PieceUnicode::get_black_rook(),
            Player::Black,
        );
        let mut black_queen = Piece::new(
            1,
            1,
            0b0001000000000000000000000000000000000000000000000000000000000000,
            "running",
            "q",
            "running",
            0x1000000000000000,
            PieceUnicode::get_black_queen(),
            Player::Black,
        );
        let mut black_king = Piece::new(
            1,
            1,
            0b100000000000000000000000000000000000000000000000000000000000,
            "running",
            "k",
            "running",
            0x0800000000000000,
            PieceUnicode::get_black_king(),
            Player::Black,
        );
        // println!("{}", (length as usize).reverse_bits() );
        let white_piece = (white_bishop | white_king) | (white_knight | white_pawn) | (white_queen | white_rook);
        let black_piece = (black_bishop | black_king) | (black_knight | black_pawn) | (black_queen | black_rook);
        let occupied = white_piece | black_piece;
        let empty= !occupied;
        // println!("{}", white_pieces)
        black_pawn.pawn_move(empty, 2);
        white_pawn.pawn_move(empty, 2);
        black_pawn.pawn_attack(white_piece);
        // white_pawn.pawn_attack(black_piece);
        self.populate( vec![white_king, white_bishop, white_knight, 
        white_pawn, white_queen, white_rook, 
        black_king, black_bishop, black_knight, 
        black_pawn, black_queen, black_rook]);
        println!("{}", self);
    }
    pub fn normal_chessboard(&mut self) {
        let mut white_pawn = Piece::new(
            1,
            1,
            0b1111111100000000,
            "white pawn",
            "P",
            "running",
            0x000000000000ff00,
            PieceUnicode::get_white_pawn(),
            Player::White,
        );
        let mut white_knight = Piece::new(
            1,
            1,
            0b1000010,
            "running",
            "N",
            "running",
            0x0000000000000042,
            PieceUnicode::get_white_knight(),
            Player::White,
        );
        let mut white_bishop = Piece::new(
            1,
            1,
            0b100100,
            "running",
            "B",
            "running",
            0x0000000000000024,
            PieceUnicode::get_white_bishop(),
            Player::White,
        );
        let mut white_rook = Piece::new(
            1,
            1,
            0b10000001,
            "running",
            "R",
            "running",
            0x0000000000000081,
            PieceUnicode::get_white_rook(),
            Player::White,
        );
        let mut white_king = Piece::new(
            1,
            1,
            0b1000,
            "running",
            "K",
            "running",
            0x0000000000000008,
            PieceUnicode::get_white_king(),
            Player::White,
        );
        let mut white_queen = Piece::new(
            1,
            1,
            0b0000000000000000000000000000000000000000000000000000000000010000,
            "running",
            "Q",
            "running",
            0x0000000000000010,
            PieceUnicode::get_white_queen(),
            Player::White,
        );
        let mut black_pawn = Piece::new(
            1,
            1,
            0b11111111000000000000000000000000000000000000000000000000,
            "running",
            "p",
            "running",
            0x00ff000000000000,
            PieceUnicode::get_black_pawn(),
            Player::Black,
        );
        let mut black_knight = Piece::new(
            1,
            1,
            0b100001000000000000000000000000000000000000000000000000000000000,
            "running",
            "n",
            "running",
            0x4200000000000000,
            PieceUnicode::get_black_knight(),
            Player::Black,
        );
        let mut black_bishop = Piece::new(
            1,
            1,
            0b10010000000000000000000000000000000000000000000000000000000000,
            "running",
            "b",
            "running",
            0x2400000000000000,
            PieceUnicode::get_black_bishop(),
            Player::Black,
        );
        let mut black_rook = Piece::new(
            1,
            1,
            0b1000000100000000000000000000000000000000000000000000000000000000,
            "running",
            "r",
            "running",
            0x8100000000000000,
            PieceUnicode::get_black_rook(),
            Player::Black,
        );
        let mut black_queen = Piece::new(
            1,
            1,
            0b0001000000000000000000000000000000000000000000000000000000000000,
            "running",
            "q",
            "running",
            0x1000000000000000,
            PieceUnicode::get_black_queen(),
            Player::Black,
        );
        let mut black_king = Piece::new(
            1,
            1,
            0b100000000000000000000000000000000000000000000000000000000000,
            "running",
            "k",
            "running",
            0x0800000000000000,
            PieceUnicode::get_black_king(),
            Player::Black,
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

    pub fn game_cycle(&mut self) {
        let mut running = true;

        while running {
            println!("{}", self);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input = input.trim().to_string().to_uppercase();

            let words: Vec<&str> = input.split_whitespace().collect();
            let all_two_chars = words.iter().all(|word| word.len() == 2);

            if input == "QUIT" {
                running = false;
                continue;
            }

            if all_two_chars && words.len() == 3 && words[1] == "TO" {
                if self.all_pieces().contains(&words[0].to_string())
                    && self.all_pieces().contains(&words[2].to_string())
                {
                    self.move_piece(words[0].to_string(), words[2].to_string());
                }
            } else if words.len() == 1 {
                if self.all_pieces().contains(&words[0].to_string()) {
                    let q = self.select_piece(words[0].to_string());
                    self.get_single(q);
                }
            } else {
                println!("Invalid input: All words must have exactly two characters.");
            }
            sleep(time::Duration::from_secs(2));
            self.clear_screen();            
        }
        self.close_program();
    }

    fn get_single(&mut self, piece: Option<Piece>) {
        match piece {
            Some(val) => println!("The piece you selected is {}\n", val),
            None => println!("You selected an empty space"),
        }
    }

    pub fn close_program(&self) {
        println!("The Chess program have closed");
    }

    fn get_hashmap(&self) -> HashMap<char, i32> {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let alpha: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let mut map = HashMap::new();
        for i in 0..numbers.len() {
            let _ = &map.insert(alpha[i], numbers[i]);
        }
        return map;
    }

    fn all_pieces(&self) -> Vec<String> {
        let mut result = Vec::new();

        for row in 1..=8 {
            for col in 'A'..='H' {
                result.push(format!("{}{}", row, col));
            }
        }

        result
    }

    fn select_piece(&mut self, value: String) -> Option<Piece> {
        if value.len() == 2 {
            let sub_strings: Vec<char> = value.chars().collect();
            let a: usize = sub_strings[0].to_digit(10).unwrap_or_default() as usize;
            let b: usize = self.get_hashmap().get(&sub_strings[1]).cloned().unwrap() as usize;
            return self._board[a - 1][b - 1];
        }
        return None;
    }

    fn move_piece(&mut self, value1: String, value2: String) {
        let sub_strings1: Vec<char> = value1.chars().collect();
        let sub_strings2: Vec<char> = value2.chars().collect();
        let a1: usize = sub_strings1[0].to_digit(10).unwrap_or_default() as usize;
        let b1: usize = self.get_hashmap().get(&sub_strings1[1]).cloned().unwrap() as usize;
        let a2: usize = sub_strings2[0].to_digit(10).unwrap_or_default() as usize;
        let b2: usize = self.get_hashmap().get(&sub_strings2[1]).cloned().unwrap() as usize;
        let val = match self._board[a1 -1][b1 - 1]{
            Some(p) => p,
            None => return
        };
        if match val._color.cmp(&self._current){Ordering::Equal => true, _ => false}{
            match self._board[a2 - 1][b2 - 1]{
                Some(_val) => {println!("There is a piece on that spot")},
                None => {
                    let _piecea = self._board[a2 - 1][b2 - 1];
                    self._board[a2 - 1][b2 - 1] = self._board[a1 - 1][b1 - 1];
                    self._board[a1 - 1][b1 - 1] = _piecea;
                    if self._current == Player::Black {
                        self._current = Player::White;
                    } else {
                        self._current = Player::Black;
                    }
                }
            }
        } else {
            println!("The correct play to play is the {} player", &self._current);
        } 
    }

    pub fn clear_screen(&self,) {
        if cfg!(windows) {
            Command::new("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }

    pub fn populate(&mut self, chess_piece: Vec<Piece>) {
        // This function takes in a vector of chess pieces 
        self._board = [[None; 8]; 8];
        for i in chess_piece.iter() {
            let val = i.decimal_to_string();
            // println!("{}", val);
            // val.chars().find(|x | x.to_digit(10) == Some(1));
            let mut indices = val.char_indices();
            for _ in 0..val.len()+1{
                // println!("Yep");
                match indices.next(){
                    None => {},
                    Some((a, b)) => {
                        if b=='1' {
                            let whole = a/8;
                            let remain = a%8;
                            // println!("{}, {}",whole, remain);
                            self._board[whole][remain] = Some(*i);
                        }
                    }
                }
            }
        }
    }

}

/// This `impl Display for BoardGeneration` block is implementing the `Display` trait for the
/// `BoardGeneration` struct. By doing this, instances of `BoardGeneration` can be formatted as a string
/// when using the `println!` macro or any other formatting macros that require the `Display` trait.
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
