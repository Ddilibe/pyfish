use std::collections::HashMap;
use bitflags::bitflags;

const FILE_NAMES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
const RANK_NAMES: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Status: u32 {
        const VALID                   = 0;
        const NO_WHITE_KING           = 1 << 0;
        const NO_BLACK_KING           = 1 << 1;
        const TOO_MANY_KINGS          = 1 << 2;
        const TOO_MANY_WHITE_PAWNS    = 1 << 3;
        const TOO_MANY_BLACK_PAWNS    = 1 << 4;
        const PAWNS_ON_BACKRANK       = 1 << 5;
        const TOO_MANY_WHITE_PIECES   = 1 << 6;
        const TOO_MANY_BLACK_PIECES   = 1 << 7;
        const BAD_CASTLING_RIGHTS     = 1 << 8;
        const INVALID_EP_SQUARE       = 1 << 9;
        const OPPOSITE_CHECK          = 1 << 10;
        const EMPTY                   = 1 << 11;
        const RACE_CHECK              = 1 << 12;
        const RACE_OVER               = 1 << 13;
        const RACE_MATERIAL           = 1 << 14;
        const TOO_MANY_CHECKERS       = 1 << 15;
        const IMPOSSIBLE_CHECK        = 1 << 16;
    }
}

pub enum Colors {
    WHITE,
    BLACK
}

pub enum ALL {}

impl ALL {
    pub fn EnPassantSpec() -> vec<&str> {
        ["legal", "fen", "xfen"]
    }

    pub fn ColorName() -> vec<&str> {
        ["white", "black"]
    }

    pub fn PieceType() -> vec<Piece> {
        [Piece::PAWN, Piece::Knight, Piece::BISHOP, Piece::ROOK, Piece::QUEEN, Piece::KING]
    }

    pub fn get_unicode_symbols() -> HashMap<char, &str> {
        let map = HashMap::new();
        map.add()
    }
    
}