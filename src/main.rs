mod engine;
mod menu;
mod piece;

use engine::game_state::GameState;
use std::io::{self, Write};

#[allow(unused_imports, dead_code)]
use engine::boardgeneration::BoardGeneration;
#[allow(unused_imports, dead_code)]
use engine::constants::{FILE_A, FILE_H, RANK1, RANK8};
#[allow(unused_imports, dead_code)]
use menu::menu::Menu;
#[allow(unused_imports, dead_code)]
use piece::piece::Piece;
#[allow(unused_imports, dead_code)]
use piece::piece::PieceUnicode;

fn main() {
    let mut game = GameState::new();
    println!("Welcome to Crab Chess!");
    println!("{}", game.board);

    loop {
        // Print current game state
        println!(
            "\nMove {}, {} to play",
            game.move_count, game.current_player
        );

        match game.status {
            engine::game_state::GameStatus::Check => println!("Check!"),
            engine::game_state::GameStatus::Checkmate => {
                println!(
                    "Checkmate! {} wins!",
                    if game.current_player == piece::piece::Player::White {
                        "Black"
                    } else {
                        "White"
                    }
                );
                break;
            }
            engine::game_state::GameStatus::Stalemate => {
                println!("Stalemate!");
                break;
            }
            engine::game_state::GameStatus::Draw => {
                println!("Draw!");
                break;
            }
            engine::game_state::GameStatus::AwaitingPromotion => {
                print!("Choose promotion piece (q/r/b/n): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();

                match game.promote_pawn(input.chars().next().unwrap_or('q')) {
                    Ok(_) => {
                        println!("\n{}", game.board);
                        continue;
                    }
                    Err(e) => {
                        println!("Invalid promotion choice: {}", e);
                        continue;
                    }
                }
            }
            engine::game_state::GameStatus::InProgress => {}
        }

        // Get move from player
        print!("Enter move (e.g., 'e2 e4') or 'quit' to exit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }

        // Parse move
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() == 1 {
            let p = game.board.select_piece(&parts[0].to_string());
            game.board.get_single(p);
            continue;
        }
        if parts.len() != 2 {
            println!("Invalid move format. Please use format 'e2 e4'");
            continue;
        }

        // Try to make the move
        match game.make_move(parts[0].to_string(), parts[1].to_string()) {
            Ok(_) => {
                println!("\n{}", game.board);
            }
            Err(e) => {
                println!("Invalid move: {}\n", e);
                println!("\n{}", game.board);
                continue;
            }
        }
    }
}
