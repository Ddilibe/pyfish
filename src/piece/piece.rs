use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Display;

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
    _image_path: &'static str,
    _hexa: i128,
    _unicode: &'static str,
    pub _color: Player,
}

impl Piece {
    pub fn new(
        id: i64,
        position: i32,
        bitwise: i128,
        name: &'static str,
        image_path: &'static str,
        decimal: i128,
        unicode: &'static str,
        color: Player
    ) -> Self {
        Self {
            _id: id,
            _position: position,
            _bitwise: bitwise,
            _name: name,
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
        write!(f, "{}", self._unicode)
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
