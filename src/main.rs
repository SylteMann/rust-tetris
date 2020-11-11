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
}

mod cli_minesweeper {
    use crate::game::Game;
    use std::error::Error;
    use std::num::ParseIntError;
    use std::string::ParseError;
    use rand::rngs::adapter::ReadError;
    use std::ffi::IntoStringError;

    fn get_coordinates_cli() -> Result<(usize, usize), String>{
        println!("Please enter coordinates in format x, y");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");
        let mut split = input.split(",");
        let x_str = split.next();
        let y_str = split.next();
        let x_result = match x_str{
            Some(R) => R.trim().parse::<usize>(),
            None => return Result::Err(String::from("Couldn't get x-value")),
        };
        let y_result = match y_str {
            Some(R) => R.trim().parse::<usize>(),
            None => return Result::Err(String::from("Couldn't get y-value"))
        };

        let x = match x_result {
            Ok(x) => x,
            Err(_) => return Result::Err(String::from("couldn't parse x-value")),
        };
        let y = match y_result {
            Ok(y) => y,
            Err(_) => return Result::Err(String::from("Couldn't parse y-value")),
        };
        Result::Ok((x, y))
    }

    fn clear_terminal() {
        print!("{}[2J", 27 as char);
    }

    pub fn start_game() {
        let mut game = Game::new(10, 10, 5);
        loop {
            clear_terminal();
            let (x, y) = match get_coordinates_cli() {
                Ok(R) => R,
                Err(E) => {
                    println!("{}", E);
                    continue;
                }
            };
            game.click_square(x, y);
            if game.has_won() {
                println!("Congratulations! You won!");
                break;
            } else if game.has_lost() {
                println!("You just lost :(");
                break;
            }
        }
    }
}

fn main() {
    self::cli_minesweeper::start_game();
}

