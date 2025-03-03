// Built-in libraries
use std::cmp::Eq;
use std::ops::BitOr;
use std::fmt::Error;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::engine::constants::FILE_A;
use crate::engine::constants::FILE_H;
// Recently built crates
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
    _alpha: &'static str,
    _image_path: &'static str,
    _hexa: u128,
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
