use std::vec;
use crate::game::Game;
use std::io::Write;


mod game;

mod cli_minesweeper;

fn main() {
    self::cli_minesweeper::start_game();
}

