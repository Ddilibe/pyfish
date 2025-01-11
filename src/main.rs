

mod piece;
mod engine;

#[warn(unused_imports, dead_code)]
use piece::piece::Piece;
use piece::piece::PieceUnicode;
use engine::boardgeneration::BoardGeneration;


fn main() {
    // let new_piece = Piece::new(
    //     1, 1, 1, "running", "running",1, PieceUnicode::get_black_bishop()
    // );
    let mut board = BoardGeneration::new();
    board.normal_chessboard();
    board.game_cycle();
    // println!("{}", new_piece);
}
