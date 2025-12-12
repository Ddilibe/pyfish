// Built-in libraries
use std::cmp::Eq;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::BitOr;

// Recently built crates
use crate::engine::constants::FILE_A;
use crate::engine::constants::FILE_B;
use crate::engine::constants::FILE_G;
use crate::engine::constants::FILE_H;
use crate::engine::constants::RANK1;
use crate::engine::constants::RANK2;
use crate::engine::constants::RANK4;
use crate::engine::constants::RANK5;
use crate::engine::constants::RANK7;
use crate::engine::constants::RANK8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Player {
    White,
    Black,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if *self == Player::Black {
            write!(f, "Black")
        } else {
            write!(f, "white")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Piece {
    pub _id: i64,
    pub _position: i32,
    pub _name: &'static str,
    pub _alpha: &'static str,
    pub _image_path: &'static str,
    pub _hexa: u128,
    pub _unicode: &'static str,
    pub _color: Player,
    pub has_moved: bool, // Track if piece has moved (for pawns and castling)
}

impl Piece {
    pub fn new(
        id: i64,
        position: i32,
        name: &'static str,
        alpha: &'static str,
        image_path: &'static str,
        decimal: u128,
        unicode: &'static str,
        color: Player,
    ) -> Self {
        Self {
            _id: id,
            _position: position,
            _name: name,
            _alpha: alpha,
            _image_path: image_path,
            _hexa: decimal,
            _unicode: unicode,
            _color: color,
            has_moved: false,
        }
    }

    pub fn decimal_to_string(self) -> String {
        let mut string = String::new();
        let mut value = self._hexa;
        while value != 0 {
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
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if cfg!(windows) {
            write!(f, "{}", self._alpha)
        } else {
            write!(f, "{}", self._unicode)
        }
    }
}

impl<'a, 'b> BitOr for Piece {
    type Output = u128;
    fn bitor(self, rhs: Self) -> Self::Output {
        self._hexa | rhs._hexa
    }
}

impl Piece {
    pub fn get_pawn_moves(
        &mut self,
        empty: u128,
        opponent_pieces: u128,
        last_move: Option<(u128, u128)>,
    ) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "p",
            "This Chess Piece is not a pawn"
        );
        self.has_moved = {
            if last_move == Some((0, 0)) {
                true
            } else {
                false
            }
        };
        let mut moves: u128 = 0;
        let position = self._hexa;

        match self._color {
            Player::White => {
                // Single push
                let single_push = (position << 8) & empty;
                moves |= single_push;

                // Double push from starting position
                if !self.has_moved && single_push != 0 {
                    moves |= (single_push << 8) & empty & (RANK4);
                }

                // Captures
                moves |= (position << 7) & opponent_pieces & !FILE_A; // Capture left
                moves |= (position << 9) & opponent_pieces & !FILE_H; // Capture right

                // En passant
                if let Some((from, to)) = last_move {
                    // Check if last move was a black pawn double push
                    if (from & RANK7) != 0 && (to & RANK5) != 0 {
                        // Add en passaRANK1ptures if our pawn is on rank 5
                        if (position & RANK5) != 0 {
                            // En passant left
                            if (position & !FILE_A) != 0 && ((to & (position >> 1)) != 0) {
                                moves |= position << 7;
                            }
                            // En passant right
                            if (position & !FILE_H) != 0 && ((to & (position << 1)) != 0) {
                                moves |= position << 9;
                            }
                        }
                    }
                }

                // Remove moves to 8th rank (handled by promotion)
                moves &= !RANK8;
            }
            Player::Black => {
                // Single push
                let single_push = (position >> 8) & empty;
                moves |= single_push;

                // Double push from starting position
                if !self.has_moved && single_push != 0 {
                    moves |= (single_push >> 8) & empty & (RANK5);
                }

                // Captures
                moves |= (position >> 9) & opponent_pieces & !FILE_A; // Capture left
                moves |= (position >> 7) & opponent_pieces & !FILE_H; // Capture right

                // En passant
                if let Some((from, to)) = last_move {
                    // Check if last move was a white pawn double push
                    if (from & RANK2) != 0 && (to & RANK4) != 0 {
                        // Add en passant captures if our pawn is on rank 4
                        if (position & RANK4) != 0 {
                            // En passant left
                            if (position & !FILE_A) != 0 && ((to & (position >> 1)) != 0) {
                                moves |= position >> 9;
                            }
                            // En passant right
                            if (position & !FILE_H) != 0 && ((to & (position << 1)) != 0) {
                                moves |= position >> 7;
                            }
                        }
                    }
                }

                // Remove moves to 1st rank (handled by promotion)
                moves &= !RANK1;
            }
        }

        self._hexa = moves;

        moves
    }

    // Separate function for pawn promotion checks
    pub fn is_promotion_move(&self, target_square: u128) -> bool {
        match self._color {
            Player::White => (target_square & RANK8) != 0,
            Player::Black => (target_square & RANK1) != 0,
        }
    }

    // Replace existing pawn_move and pawn_attack with a call to get_pawn_moves
    pub fn pawn_move(&mut self, empty: u128, number: u128) {
        self.get_pawn_moves(empty, 0, None);
    }

    pub fn pawn_attack(&self, opponent_pieces: u128) -> u128 {
        self.get_pawn_attacks(opponent_pieces, None)
    }

    pub fn knight_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(
            self._alpha.to_lowercase(),
            "n",
            "This Chess Piece is not a knight"
        );
        let position = self._hexa;

        // Knight move patterns: 8 possible moves
        let moves = [
            (position << 17) & !FILE_H,           // Up 2, Right 1
            (position << 15) & !FILE_A,           // Up 2, Left 1
            (position << 10) & !FILE_H & !FILE_G, // Up 1, Right 2
            (position << 6) & !FILE_A & !FILE_B,  // Up 1, Left 2
            (position >> 6) & !FILE_H & !FILE_G,  // Down 1, Right 2
            (position >> 10) & !FILE_A & !FILE_B, // Down 1, Left 2
            (position >> 15) & !FILE_H,           // Down 2, Right 1
            (position >> 17) & !FILE_A,           // Down 2, Left 1
        ];

        // Combine all possible moves and filter by empty squares and opponent pieces
        self._hexa = moves.iter().fold(0, |acc, &m| acc | m) & (empty | opponent_pieces);
    }

    pub fn bishop_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(
            self._alpha.to_lowercase(),
            "b",
            "This Chess Piece is not a bishop"
        );
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
    }

    pub fn rook_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(
            self._alpha.to_lowercase(),
            "r",
            "This Chess Piece is not a rook"
        );
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
    }

    pub fn queen_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(
            self._alpha.to_lowercase(),
            "q",
            "This Chess Piece is not a queen"
        );
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
    }

    pub fn king_moves(&mut self, empty: u128, opponent_pieces: u128) {
        assert_eq!(
            self._alpha.to_lowercase(),
            "k",
            "This Chess Piece is not a king"
        );
        let position = self._hexa;

        // All possible king moves
        let moves = [
            (position << 8),           // North
            (position >> 8),           // South
            (position << 1) & !FILE_H, // East
            (position >> 1) & !FILE_A, // West
            (position << 9) & !FILE_H, // Northeast
            (position << 7) & !FILE_A, // Northwest
            (position >> 7) & !FILE_H, // Southeast
            (position >> 9) & !FILE_A, // Southwest
        ];

        // Combine all possible moves and filter by empty squares and opponent pieces
        self._hexa = moves.iter().fold(0, |acc, &m| acc | m) & (empty | opponent_pieces);
    }

    pub fn get_pawn_attacks(
        &self,
        opponent_pieces: u128,
        last_move: Option<(u128, u128)>,
    ) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "p",
            "This Chess Piece is not a pawn"
        );
        let mut attacks: u128 = 0;
        let position = self._hexa;

        match self._color {
            Player::White => {
                // Captures
                attacks |= (position << 7) & opponent_pieces & !FILE_A; // Capture left
                attacks |= (position << 9) & opponent_pieces & !FILE_H; // Capture right

                // En passant
                if let Some((from, to)) = last_move {
                    // Check if last move was a black pawn double push
                    if (from & RANK7) != 0 && (to & RANK5) != 0 {
                        // Add en passant captures if our pawn is on rank 5
                        if (position & RANK5) != 0 {
                            // En passant left
                            if (position & !FILE_A) != 0 && ((to & (position >> 1)) != 0) {
                                attacks |= position << 7;
                            }
                            // En passant right
                            if (position & !FILE_H) != 0 && ((to & (position << 1)) != 0) {
                                attacks |= position << 9;
                            }
                        }
                    }
                }
            }
            Player::Black => {
                // Captures
                attacks |= (position >> 9) & opponent_pieces & !FILE_A; // Capture left
                attacks |= (position >> 7) & opponent_pieces & !FILE_H; // Capture right

                // En passant
                if let Some((from, to)) = last_move {
                    // Check if last move was a white pawn double push
                    if (from & RANK2) != 0 && (to & RANK4) != 0 {
                        // Add en passant captures if our pawn is on rank 4
                        if (position & RANK4) != 0 {
                            // En passant left
                            if (position & !FILE_A) != 0 && ((to & (position >> 1)) != 0) {
                                attacks |= position >> 9;
                            }
                            // En passant right
                            if (position & !FILE_H) != 0 && ((to & (position << 1)) != 0) {
                                attacks |= position >> 7;
                            }
                        }
                    }
                }
            }
        }
        attacks
    }

    pub fn get_knight_attacks(&self, opponent_pieces: u128) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "n",
            "This Chess Piece is not a knight"
        );
        let position = self._hexa;
        let mut attacks: u128 = 0;

        let moves = [
            (position << 17) & !FILE_H,           // Up 2, Right 1
            (position << 15) & !FILE_A,           // Up 2, Left 1
            (position << 10) & !FILE_H & !FILE_G, // Up 1, Right 2
            (position << 6) & !FILE_A & !FILE_B,  // Up 1, Left 2
            (position >> 6) & !FILE_H & !FILE_G,  // Down 1, Right 2
            (position >> 10) & !FILE_A & !FILE_B, // Down 1, Left 2
            (position >> 15) & !FILE_H,           // Down 2, Right 1
            (position >> 17) & !FILE_A,           // Down 2, Left 1
        ];

        for &m in moves.iter() {
            attacks |= m & opponent_pieces;
        }
        attacks
    }

    pub fn get_bishop_attacks(&self, all_pieces: u128, opponent_pieces: u128) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "b",
            "This Chess Piece is not a bishop"
        );
        let mut attacks: u128 = 0;
        let position = self._hexa;

        // Northeast diagonal
        let mut pos = position;
        while (pos & FILE_H) == 0 {
            pos <<= 9;
            if pos == 0 { break; } // Check for overflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // Northwest diagonal
        let mut pos = position;
        while (pos & FILE_A) == 0 {
            pos <<= 7;
            if pos == 0 { break; } // Check for overflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // Southeast diagonal
        let mut pos = position;
        while (pos & FILE_H) == 0 {
            pos >>= 7;
            if pos == 0 { break; } // Check for underflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // Southwest diagonal
        let mut pos = position;
        while (pos & FILE_A) == 0 {
            pos >>= 9;
            if pos == 0 { break; } // Check for underflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        attacks
    }

    pub fn get_rook_attacks(&self, all_pieces: u128, opponent_pieces: u128) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "r",
            "This Chess Piece is not a rook"
        );
        let mut attacks: u128 = 0;
        let position = self._hexa;

        // North
        let mut pos = position;
        while pos != 0 {
            pos <<= 8;
            if pos == 0 { break; } // Check for overflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // South
        let mut pos = position;
        while pos != 0 {
            pos >>= 8;
            if pos == 0 { break; } // Check for underflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // East
        let mut pos = position;
        while (pos & FILE_H) == 0 {
            pos <<= 1;
            if pos == 0 { break; } // Check for overflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        // West
        let mut pos = position;
        while (pos & FILE_A) == 0 {
            pos >>= 1;
            if pos == 0 { break; } // Check for underflow
            if (pos & opponent_pieces) != 0 {
                attacks |= pos;
                break;
            }
            if (pos & all_pieces) != 0 {
                break;
            }
        }

        attacks
    }

    pub fn get_queen_attacks(&self, all_pieces: u128, opponent_pieces: u128) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "q",
            "This Chess Piece is not a queen"
        );
        // Queen attacks combine rook and bishop attacks
        let bishop_attacks = self.get_bishop_attacks(all_pieces, opponent_pieces);
        let rook_attacks = self.get_rook_attacks(all_pieces, opponent_pieces);

        bishop_attacks | rook_attacks
    }

    pub fn get_king_attacks(&self, opponent_pieces: u128) -> u128 {
        assert_eq!(
            self._alpha.to_lowercase(),
            "k",
            "This Chess Piece is not a king"
        );
        let position = self._hexa;
        let mut attacks: u128 = 0;

        // All possible king moves (one square in any direction)
        let moves = [
            (position << 8),           // North
            (position >> 8),           // South
            (position << 1) & !FILE_H, // East
            (position >> 1) & !FILE_A, // West
            (position << 9) & !FILE_H, // Northeast
            (position << 7) & !FILE_A, // Northwest
            (position >> 7) & !FILE_H, // Southeast
            (position >> 9) & !FILE_A, // Southwest
        ];

        for &m in moves.iter() {
            attacks |= m & opponent_pieces;
        }
        attacks
    }

    // Helper method to get all possible moves for any piece
    pub fn get_moves(&mut self, empty: u128, opponent_pieces: u128) {
        match self._alpha.to_lowercase().as_str() {
            "p" => self.pawn_move(empty, opponent_pieces),
            "n" => self.knight_moves(empty, opponent_pieces),
            "b" => self.bishop_moves(empty, opponent_pieces),
            "r" => self.rook_moves(empty, opponent_pieces),
            "q" => self.queen_moves(empty, opponent_pieces),
            "k" => self.king_moves(empty, opponent_pieces),
            _ => panic!("Unknown piece type"),
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
