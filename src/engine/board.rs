use crate::piece::piece::{Piece, PieceUnicode, Player};
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::io;

const BOARD_SIZE: usize = 8;
type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

lazy_static::lazy_static! {
    static ref FILE_MAP: HashMap<char, usize> = {
        let mut m = HashMap::new();
        let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        for (i, c) in files.iter().enumerate() {
            m.insert(*c, i + 1);
        }
        m
    };
}

pub struct ChessBoard {
    board: Board,
    current_player: Player,
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::Black,
        }
    }

    pub fn setup_standard_board(&mut self) {
        fn create_piece(symbol: PieceUnicode, player: Player) -> Piece {
            Piece::new(
                1, 1, 1, "running", "running", 1, symbol, player,
            )
        }

        // White pieces
        let white_pawn = create_piece(PieceUnicode::get_white_pawn(), Player::White);
        let white_rook = create_piece(PieceUnicode::get_white_rook(), Player::White);
        let white_knight = create_piece(PieceUnicode::get_white_knight(), Player::White);
        let white_bishop = create_piece(PieceUnicode::get_white_bishop(), Player::White);
        let white_queen = create_piece(PieceUnicode::get_white_queen(), Player::White);
        let white_king = create_piece(PieceUnicode::get_white_king(), Player::White);

        // Black pieces
        let black_pawn = create_piece(PieceUnicode::get_black_pawn(), Player::Black);
        let black_rook = create_piece(PieceUnicode::get_black_rook(), Player::Black);
        let black_knight = create_piece(PieceUnicode::get_black_knight(), Player::Black);
        let black_bishop = create_piece(PieceUnicode::get_black_bishop(), Player::Black);
        let black_queen = create_piece(PieceUnicode::get_black_queen(), Player::Black);
        let black_king = create_piece(PieceUnicode::get_black_king(), Player::Black);

        self.board = [
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
            [Some(black_pawn); BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [None; BOARD_SIZE],
            [Some(white_pawn); BOARD_SIZE],
            [
                Some(white_rook),
                Some(white_knight),
                Some(white_bishop),
                Some(white_queen),
                Some(white_king),
                Some(white_bishop),
                Some(white_knight),
                Some(white_rook),
            ],
        ];
    }

    pub fn game_loop(&mut self) {
        let mut running = true;

        while running {
            println!("{}", self);

            match self.read_input() {
                Ok(Command::Quit) => running = false,
                Ok(Command::Move(from, to)) => self.move_piece(&from, &to),
                Ok(Command::Select(position)) => self.display_piece_info(&position),
                Err(err) => println!("{}", err),
            }
        }

        println!("Chess program has exited");
    }

    fn read_input(&self) -> Result<Command, String> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|_| "Failed to read input".to_string())?;

        let input = input.trim().to_uppercase();
        if input == "QUIT" {
            return Ok(Command::Quit);
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            [position] => Ok(Command::Select(position.to_string())),
            [from, "TO", to] => Ok(Command::Move(from.to_string(), to.to_string())),
            _ => Err("Invalid command format".to_string()),
        }
    }

    fn display_piece_info(&self, position: &str) {
        match self.position_to_indices(position) {
            Ok((x, y)) => {
                if let Some(piece) = self.board[x][y] {
                    println!("Selected piece: {}\n", piece);
                } else {
                    println!("No piece at {}\n", position);
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    fn position_to_indices(&self, position: &str) -> Result<(usize, usize), String> {
        let mut chars = position.chars();
        let rank = chars.next()
            .and_then(|c| c.to_digit(10))
            .ok_or_else(|| format!("Invalid rank in position: {}", position))?;
        
        let file = chars.next()
            .ok_or_else(|| format!("Invalid file in position: {}", position))?;

        if rank < 1 || rank > 8 {
            return Err(format!("Invalid rank number: {}", rank));
        }

        let x = (rank - 1) as usize;
        let y = FILE_MAP.get(&file)
            .ok_or_else(|| format!("Invalid file character: {}", file))? - 1;

        Ok((x, y))
    }

    fn move_piece(&mut self, from: &str, to: &str) {
        match (self.position_to_indices(from), self.position_to_indices(to)) {
            (Ok((from_x, from_y)), Ok((to_x, to_y))) => {
                if self.board[to_x][to_y].is_some() {
                    println!("Cannot move to occupied square");
                    return;
                }
                
                self.board[to_x][to_y] = self.board[from_x][from_y].take();
                self.current_player = self.current_player.opposite();
            }
            (Err(err), _) | (_, Err(err)) => println!("{}", err),
        }
    }

    fn all_valid_positions(&self) -> Vec<String> {
        (1..=8)
            .flat_map(|rank| ('A'..='H').map(move |file| format!("{}{}", rank, file)))
            .collect()
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "   A B C D E F G H")?;
        
        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{} ", i + 1)?;
            for piece in row {
                match piece {
                    Some(p) => write!(f, "|{}", p)?,
                    None => write!(f, "| ")?,
                }
            }
            writeln!(f, "|")?;
        }
        
        Ok(())
    }
}

enum Command {
    Quit,
    Move(String, String),
    Select(String),
}

impl Player {
    fn opposite(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}