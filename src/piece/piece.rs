// Built-in libraries
use std::cmp::Eq;
use std::ops::BitOr;
use std::fmt::Error;
use std::fmt::Display;
use std::fmt::Formatter;

// Recently built crates
use crate::engine::constants::FILE_A;
use crate::engine::constants::FILE_B;
use crate::engine::constants::FILE_G;
use crate::engine::constants::FILE_H;
use crate::engine::constants::RANK1;
use crate::engine::constants::RANK8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player {
    White,
    Black,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self == Player::Black{
            write!(f, "Black")
        } else {
            write!(f, "white")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Piece {
    _id: i64,
    _position: i32,
    pub _bitwise: i128,
    _name: &'static str,
    pub _alpha: &'static str,
    _image_path: &'static str,
    pub _hexa: u128,
    _unicode: &'static str,
    pub _color: Player,
}

impl Piece {
    pub fn new(
        id: i64,
        position: i32,
        bitwise: i128,
        name: &'static str,
        alpha: &'static str,
        image_path: &'static str,
        decimal: u128,
        unicode: &'static str,
        color: Player
    ) -> Self {
        Self {
            _id: id,
            _position: position,
            _bitwise: bitwise,
            _name: name,
            _alpha: alpha,
            _image_path: image_path,
            _hexa: decimal,
            _unicode: unicode,
            _color: color
        }
    }

    pub fn decimal_to_string(self) -> String {
        let mut string = String::new();
        let mut value = self._bitwise;
        while value != 0{
            string += (value % 2).to_string().as_str();
            value = value / 2;
        }
        // println!("{}", string);
        while string.len() < 64 {
            string = string + &"0".to_owned();
        }
        return string.chars().rev().collect();
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        if cfg!(windows) {
            write!(f, "{}", self._alpha)
        } else {
            write!(f, "{}", self._unicode)
        }
    }
}

impl <'a, 'b> BitOr for Piece {
    type Output = u128;
    fn bitor(self, rhs: Self) -> Self::Output {
        self._hexa | rhs._hexa
    }
}

impl Piece {
    pub fn pawn_move(&mut self, empty: u128, number: i8) {
        // This function is used to create moves for the pawns
        assert!(number == 1 || number == 2, "The number variable has to be either one or two");
        if self._alpha.to_lowercase() == "p" {
            match self._color {
                Player::White => {
                    self._hexa = (self._hexa << (8 * number)) & empty & !RANK8;
                    self._bitwise = self._hexa as i128;
                },
                Player::Black => {
                    self._hexa = (self._hexa >> (8 * number)) & empty & !RANK1;
                    self._bitwise = self._hexa as i128;
                }
            }
        }
    }
    pub fn pawn_attack(&mut self, color: u128) {
        assert_eq!(self._alpha.to_lowercase(), "p", "This Chess Piece is not a pawn");
        match self._color {
            Player::White => {
                self._hexa = (self._hexa << 9) & color & !RANK8 & !FILE_H;
                self._bitwise = self._hexa as i128;
                println!("This is white {}", self.decimal_to_string());
            }
            Player::Black => {
                self._hexa = (self._hexa >> 9) & color & !RANK1 & !FILE_A;
                self._bitwise = self._hexa as i128;
                println!("This is black {}, {}", self.decimal_to_string(), color);
            }
        }
    }

    pub fn knight_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(self._alpha.to_lowercase(), "n", "This Chess Piece is not a knight");
        let position = self._hexa;
        
        // Knight move patterns: 8 possible moves
        let moves = [
            (position << 17) & !FILE_H,  // Up 2, Right 1
            (position << 15) & !FILE_A,  // Up 2, Left 1
            (position << 10) & !FILE_H & !FILE_G,  // Up 1, Right 2
            (position << 6) & !FILE_A & !FILE_B,   // Up 1, Left 2
            (position >> 6) & !FILE_H & !FILE_G,   // Down 1, Right 2
            (position >> 10) & !FILE_A & !FILE_B,  // Down 1, Left 2
            (position >> 15) & !FILE_H,  // Down 2, Right 1
            (position >> 17) & !FILE_A   // Down 2, Left 1
        ];

        // Combine all possible moves and filter by empty squares and opponent pieces
        self._hexa = moves.iter().fold(0, |acc, &m| acc | m) & (empty | opponent_pieces);
        self._bitwise = self._hexa as i128;
    }

    pub fn bishop_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(self._alpha.to_lowercase(), "b", "This Chess Piece is not a bishop");
        let mut moves: u128 = 0;
        let position = self._hexa;
        
        // Northeast diagonal
        let mut pos = position;
        while pos != 0 && (pos & FILE_H) == 0 {
            pos = (pos << 9) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // Northwest diagonal
        let mut pos = position;
        while pos != 0 && (pos & FILE_A) == 0 {
            pos = (pos << 7) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // Southeast diagonal
        let mut pos = position;
        while pos != 0 && (pos & FILE_H) == 0 {
            pos = (pos >> 7) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // Southwest diagonal
        let mut pos = position;
        while pos != 0 && (pos & FILE_A) == 0 {
            pos = (pos >> 9) & (empty | opponent_pieces);
            moves |= pos;
        }

        self._hexa = moves;
        self._bitwise = self._hexa as i128;
    }

    pub fn rook_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(self._alpha.to_lowercase(), "r", "This Chess Piece is not a rook");
        let mut moves: u128 = 0;
        let position = self._hexa;
        
        // North
        let mut pos = position;
        while pos != 0 {
            pos = (pos << 8) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // South
        let mut pos = position;
        while pos != 0 {
            pos = (pos >> 8) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // East
        let mut pos = position;
        while pos != 0 && (pos & FILE_H) == 0 {
            pos = (pos << 1) & (empty | opponent_pieces);
            moves |= pos;
        }
        
        // West
        let mut pos = position;
        while pos != 0 && (pos & FILE_A) == 0 {
            pos = (pos >> 1) & (empty | opponent_pieces);
            moves |= pos;
        }

        self._hexa = moves;
        self._bitwise = self._hexa as i128;
    }

    pub fn queen_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(self._alpha.to_lowercase(), "q", "This Chess Piece is not a queen");
        // Queen combines rook and bishop moves
        let position = self._hexa;
        
        // First get bishop-like moves
        self.bishop_moves(empty, opponent_pieces);
        let bishop_moves = self._hexa;
        
        // Reset position
        self._hexa = position;
        
        // Then get rook-like moves
        self.rook_moves(empty, opponent_pieces);
        let rook_moves = self._hexa;
        
        // Combine both move sets
        self._hexa = bishop_moves | rook_moves;
        self._bitwise = self._hexa as i128;
    }

    pub fn king_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(self._alpha.to_lowercase(), "k", "This Chess Piece is not a king");
        let position = self._hexa;
        
        // All possible king moves
        let moves = [
            (position << 8),  // North
            (position >> 8),  // South
            (position << 1) & !FILE_H,  // East
            (position >> 1) & !FILE_A,  // West
            (position << 9) & !FILE_H,  // Northeast
            (position << 7) & !FILE_A,  // Northwest
            (position >> 7) & !FILE_H,  // Southeast
            (position >> 9) & !FILE_A   // Southwest
        ];

        // Combine all possible moves and filter by empty squares and opponent pieces
        self._hexa = moves.iter().fold(0, |acc, &m| acc | m) & (empty | opponent_pieces);
        self._bitwise = self._hexa as i128;
    }

    // Helper method to get all possible moves for any piece
    pub fn get_moves(&mut self, empty: u128, opponent_pieces: u128) {
        match self._alpha.to_lowercase().as_str() {
            "p" => self.pawn_attack(opponent_pieces),
            "n" => self.knight_moves(empty, opponent_pieces),
            "b" => self.bishop_moves(empty, opponent_pieces),
            "r" => self.rook_moves(empty, opponent_pieces),
            "q" => self.queen_moves(empty, opponent_pieces),
            "k" => self.king_moves(empty, opponent_pieces),
            _ => panic!("Unknown piece type")
        }
    }
}

pub enum PieceUnicode {}

impl PieceUnicode {
    pub fn get_white_pawn() -> &'static str {
        // if is_x86_feature_detected!()
        "\u{2659}"
    }
    pub fn get_white_knight() -> &'static str {
        "\u{2658}"
    }
    pub fn get_white_bishop() -> &'static str {
        "\u{2657}"
    }
    pub fn get_white_rook() -> &'static str {
        "\u{2656}"
    }
    pub fn get_white_queen() -> &'static str {
        "\u{2655}"
    }
    pub fn get_white_king() -> &'static str {
        "\u{2654}"
    }
    pub fn get_black_pawn() -> &'static str {
        "\u{265F}"
    }
    pub fn get_black_knight() -> &'static str {
        "\u{265E}"
    }
    pub fn get_black_bishop() -> &'static str {
        "\u{265D}"
    }
    pub fn get_black_rook() -> &'static str {
        "\u{265C}"
    }
    pub fn get_black_queen() -> &'static str {
        "\u{265B}"
    }
    pub fn get_black_king() -> &'static str {
        "\u{265A}"
    }
}
