

mod piece;
mod engine;
mod menu;

#[warn(unused_imports, dead_code)]
// use piece::piece::Piece;
// use piece::piece::PieceUnicode;
use engine::boardgeneration::BoardGeneration;
// use menu::menu::Menu;


fn main() {
    // let new_piece = Piece::new(
    //     1, 1, 1, "running", "running",1, PieceUnicode::get_black_bishop()
    // );
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{}", sparkle_heart);
    let mut board = BoardGeneration::new();
    board.new_normal_chessboard();
    // board.normal_chessboard();
    // board.game_cycle();0
    // println!("{}", new_piece);
    // let value = 0x00AB00000;
    // println!("{}", value | value);
    // let m = Menu();
    // m.start();
}
