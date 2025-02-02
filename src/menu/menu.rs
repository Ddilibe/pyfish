use std::io;
use crate::engine;
use crate::menu;
use menu::newgame::NewGame;
use engine::boardgeneration::BoardGeneration;


pub struct Menu();

impl Menu {
    pub fn start(&self) {
        println!("**************************");
        println!("**   Welcome to Magnus  **");
        println!("**************************\n\n");
        loop {
            println!("**************************");
            println!("** Menu                 **");
            println!("** 1. Start New Game    **");
            println!("** 2. Load Saved Game   **");
            println!("** 3. LeaderBoard       **");
            println!("** 4. Settings          **");
            println!("** 5. Help              **");
            println!("** 6. Exit              **");
            println!("**************************\n");
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Failed to read line");
            let x: i32 = number.trim().parse().expect("Please type a number!");
            match x {
                1 => {
                    let mut board = BoardGeneration::new();
                    // board.normal_chessboard();
                    // board.game_cycle();
                    let mut new_game = NewGame{board: & mut board};
                    new_game.start();
                }
                2 => println!("Option is not avaliable right Now"),
                3 => println!("Option is not avaliable right Now"),
                4 => println!("Option is not avaliable right Now"),
                5 => println!("Option is not avaliable right Now"),
                6 => break,
                _ => println!("Invalid option. Please select the appropriate option")
            }
            print!("\n");
        }
    }
}