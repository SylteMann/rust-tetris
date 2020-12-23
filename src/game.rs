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

    pub fn has_lost(&self) -> bool { self.game_board.has_hit_mine() }

    pub fn click_square(&mut self, x: usize, y:usize) {
        if !self.game_board.has_hit_mine() {
            self.game_board.click_square(x, y);
            self.print_display_board();
        } else {
            println!("Game over!");
        }
    }

    pub fn flag_field(&mut self, x: usize, y: usize) {
        if !self.game_board.has_hit_mine() {
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
