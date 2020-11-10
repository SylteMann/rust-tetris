use std::vec;
use crate::game::Game;


mod game {
    mod board;
    use board::Board;

    pub(crate) struct Game {
        game_board: Board,
    }

    impl Game {
        pub(crate) fn new(x: usize, y: usize, mines: usize) -> Game {
            let mut game_board = Board::new(x, y, mines);
            Game {
                game_board,
            }
        }

        pub fn has_won(&self) -> bool {
            self.game_board.has_won()
        }

        pub fn click_square(&mut self, x: usize, y:usize) {
            if !self.game_board.mine_hit() {
                self.game_board.click_square(x, y);
                self.print_display_board();
            } else {
                println!("Game over!");
            }
        }

        pub fn flag_field(&mut self, x: usize, y: usize) {
            if !self.game_board.mine_hit() {
                self.game_board.toggle_flag(x, y);
                self.print_display_board();
            }
        }

        pub(crate) fn print_display_board(&self) {
            self.game_board.print_display_board();
        }

        pub fn print_revealed_board(&self) {
            self.game_board.print_revealed_board();
        }
    }
}


fn main() {
    let mut game: game::Game = Game::new(10, 10, 2);
    game.print_revealed_board();
    println!();
    game.click_square(1, 1);
    game.click_square(2,2);
    game.click_square(3,3);
    eprintln!("game.has_won() = {:#?}", game.has_won());
}

