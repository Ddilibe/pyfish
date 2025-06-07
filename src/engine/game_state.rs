use crate::piece::piece::{Piece, Player};
use super::boardgeneration::BoardGeneration;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameStatus {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
    Draw,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GameState {
    pub board: BoardGeneration,
    pub current_player: Player,
    pub status: GameStatus,
    pub move_count: u32,
    pub last_move: Option<(String, String)>, // From square, To square
}

impl GameState {
    pub fn new() -> Self {
        let mut board = BoardGeneration::new();
        board.new_normal_chessboard();
        
        Self {
            board,
            current_player: Player::White,
            status: GameStatus::InProgress,
            move_count: 1,
            last_move: None,
        }
    }

    // Get all pieces of a specific color
    pub fn get_pieces_for_player(&self, player: Player) -> Vec<Piece> {
        let mut pieces = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board._board[row][col] {
                    if piece._color == player {
                        pieces.push(piece);
                    }
                }
            }
        }
        pieces
    }

    // Get the king's position for a player
    pub fn get_king_position(&self, player: Player) -> Option<(usize, usize)> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board._board[row][col] {
                    if piece._color == player && piece._alpha.to_lowercase() == "k" {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }

    // Check if a player is in check
    pub fn is_in_check(&self, player: Player) -> bool {
        // Get king's position
        let king_pos = match self.get_king_position(player) {
            Some(pos) => pos,
            None => return false, // No king found (shouldn't happen in a valid game)
        };

        // Get all opponent pieces
        let opponent = if player == Player::White { Player::Black } else { Player::White };
        let opponent_pieces = self.get_pieces_for_player(opponent);

        // Get empty squares and opponent pieces as bitboards
        let mut empty: u128 = 0;
        let mut opponent_squares: u128 = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                let square = 1u128 << (row * 8 + col);
                if self.board._board[row][col].is_none() {
                    empty |= square;
                } else if self.board._board[row][col].unwrap()._color == opponent {
                    opponent_squares |= square;
                }
            }
        }

        // Check if any opponent piece can attack the king
        for mut piece in opponent_pieces {
            piece.get_moves(empty, opponent_squares);
            if (piece._hexa & (1u128 << (king_pos.0 * 8 + king_pos.1))) != 0 {
                return true;
            }
        }

        false
    }

    // Check if a player is in checkmate
    pub fn is_checkmate(&self, player: Player) -> bool {
        // First check if the player is in check
        if !self.is_in_check(player) {
            return false;
        }

        // Get all player's pieces
        let pieces = self.get_pieces_for_player(player);
        
        // Get empty squares and opponent pieces as bitboards
        let mut empty: u128 = 0;
        let opponent = if player == Player::White { Player::Black } else { Player::White };
        let mut opponent_squares: u128 = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                let square = 1u128 << (row * 8 + col);
                if self.board._board[row][col].is_none() {
                    empty |= square;
                } else if self.board._board[row][col].unwrap()._color == opponent {
                    opponent_squares |= square;
                }
            }
        }

        // Try all possible moves for each piece
        for mut piece in pieces {
            piece.get_moves(empty, opponent_squares);
            if piece._hexa != 0 {
                // TODO: For each move, we need to verify it actually gets us out of check
                // This requires implementing move simulation
                return false;
            }
        }

        true
    }

    // Check if a player is in stalemate
    pub fn is_stalemate(&self, player: Player) -> bool {
        // If the player is in check, it's not a stalemate
        if self.is_in_check(player) {
            return false;
        }

        // Get all player's pieces
        let pieces = self.get_pieces_for_player(player);
        
        // Get empty squares and opponent pieces as bitboards
        let mut empty: u128 = 0;
        let opponent = if player == Player::White { Player::Black } else { Player::White };
        let mut opponent_squares: u128 = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                let square = 1u128 << (row * 8 + col);
                if self.board._board[row][col].is_none() {
                    empty |= square;
                } else if self.board._board[row][col].unwrap()._color == opponent {
                    opponent_squares |= square;
                }
            }
        }

        // Check if any piece has a legal move
        for mut piece in pieces {
            piece.get_moves(empty, opponent_squares);
            if piece._hexa != 0 {
                // For each potential move, verify it doesn't put or leave the king in check
                let piece_pos = self.find_piece_position(&piece).unwrap();
                let mut potential_moves = Vec::new();
                
                // Convert bitboard of moves to coordinates
                for row in 0..8 {
                    for col in 0..8 {
                        let square = 1u128 << (row * 8 + col);
                        if (piece._hexa & square) != 0 {
                            potential_moves.push((row, col));
                        }
                    }
                }

                // Check if any move is legal (doesn't leave king in check)
                for move_pos in potential_moves {
                    if self.is_safe_move(piece_pos, move_pos) {
                        return false; // Found a legal move, not stalemate
                    }
                }
            }
        }

        // No legal moves found, it's a stalemate
        true
    }

    // Helper function to find a piece's position on the board
    fn find_piece_position(&self, piece: &Piece) -> Option<(usize, usize)> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(board_piece) = self.board._board[row][col] {
                    if board_piece._alpha == piece._alpha && 
                       board_piece._color == piece._color && 
                       board_piece._hexa == piece._hexa {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }

    // Update game status
    pub fn update_status(&mut self) {
        if self.is_checkmate(self.current_player) {
            self.status = GameStatus::Checkmate;
        } else if self.is_in_check(self.current_player) {
            self.status = GameStatus::Check;
        } else if self.is_stalemate(self.current_player) {
            self.status = GameStatus::Stalemate;
        } else {
            self.status = GameStatus::InProgress;
        }
    }

    // Convert algebraic notation (e.g., "e4") to board coordinates
    fn algebraic_to_coords(&self, algebraic: &str) -> Result<(usize, usize), &'static str> {
        if algebraic.len() != 2 {
            return Err("Invalid algebraic notation");
        }
        
        let file = algebraic.chars().nth(0).unwrap();
        let rank = algebraic.chars().nth(1).unwrap();
        
        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return Err("Invalid algebraic notation");
        }
        
        let col = (file as u8 - b'a') as usize;
        let row = 7 - (rank as u8 - b'1') as usize; // Convert to 0-7 range, flipped because our board is zero-indexed from top
        
        Ok((row, col))
    }

    // Convert board coordinates to algebraic notation
    fn coords_to_algebraic(&self, row: usize, col: usize) -> String {
        let file = (b'a' + col as u8) as char;
        let rank = (b'1' + (7 - row) as u8) as char; // Convert back to 1-8 range, flipped because our board is zero-indexed from top
        format!("{}{}", file, rank)
    }

    // Check if a move is valid for a given piece
    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), piece: Piece) -> bool {
        let mut empty: u128 = 0;
        let mut opponent_squares: u128 = 0;
        
        // Build bitboards for empty squares and opponent pieces
        for row in 0..8 {
            for col in 0..8 {
                let square = 1u128 << (row * 8 + col);
                if self.board._board[row][col].is_none() {
                    empty |= square;
                } else if self.board._board[row][col].unwrap()._color != piece._color {
                    opponent_squares |= square;
                }
            }
        }

        // Get all possible moves for the piece
        let mut piece_copy = piece;
        piece_copy.get_moves(empty, opponent_squares);

        // Check if the target square is in the possible moves
        let target_square = 1u128 << (to.0 * 8 + to.1);
        (piece_copy._hexa & target_square) != 0
    }

    // Simulate a move and check if it leaves the king in check
    fn is_safe_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let mut board_copy = self.board.clone();
        let moving_piece = board_copy._board[from.0][from.1].unwrap();
        
        // Make the move on the copy
        board_copy._board[to.0][to.1] = Some(moving_piece);
        board_copy._board[from.0][from.1] = None;
        
        // Check if the move leaves the king in check
        let mut game_copy = Self {
            board: board_copy,
            current_player: self.current_player,
            status: self.status,
            move_count: self.move_count,
            last_move: self.last_move,
        };
        
        !game_copy.is_in_check(self.current_player)
    }

    // Make a move
    pub fn make_move(&mut self, from: String, to: String) -> Result<(), &'static str> {
        // Convert algebraic notation to coordinates
        let from_coords = self.algebraic_to_coords(&from)?;
        let to_coords = self.algebraic_to_coords(&to)?;
        
        // Get the piece at the starting position
        let piece = match self.board._board[from_coords.0][from_coords.1] {
            Some(p) => p,
            None => return Err("No piece at starting position"),
        };
        
        // Check if it's the correct player's turn
        if piece._color != self.current_player {
            return Err("Not your piece to move");
        }
        
        // Check if the move is valid for this piece
        if !self.is_valid_move(from_coords, to_coords, piece) {
            return Err("Invalid move for this piece");
        }
        
        // Check if the move leaves the king in check
        if !self.is_safe_move(from_coords, to_coords) {
            return Err("Move would leave king in check");
        }
        
        // Make the move
        self.board._board[to_coords.0][to_coords.1] = Some(piece);
        self.board._board[from_coords.0][from_coords.1] = None;
        
        // Update last move
        self.last_move = Some((from, to));
        
        // Switch current player
        self.current_player = if self.current_player == Player::White {
            Player::Black
        } else {
            Player::White
        };
        
        // Update move count if it was black's move
        if self.current_player == Player::White {
            self.move_count += 1;
        }
        
        // Update game status
        self.update_status();
        
        Ok(())
    }
} 