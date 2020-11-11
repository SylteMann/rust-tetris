use std::slice::Iter;

use rand::{Rng, thread_rng};

pub struct Board {
    mines_board_mask: Vec<Vec<bool>>,
    surrounding_mines_board: Vec<Vec<u8>>,
    display_board: Vec<Vec<char>>,
    hit_mine: bool,
}

impl Board {
    pub fn new(x: usize, y: usize, mines: usize) -> Board {
        let mut mines_board_mask = Board::generate_mines_board_mask(x, y, mines);
        let mut surrounding_mines_board = Board::compute_surrounding_mines_board(&mines_board_mask);
        let mut display_board = vec![vec!['?'; x]; y];
        Board {
            mines_board_mask,
            surrounding_mines_board,
            display_board,
            hit_mine: false,
        }
    }

    pub fn get_display_board(&self) -> &Vec<Vec<char>> {
        &self.display_board
    }

    pub fn has_won(&self) -> bool {
        for (y, line) in self.display_board.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if field == &'?' && !self.mines_board_mask[y][x] {
                    return false
                }
            }
        }
        true
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        let mut display_field = self.display_board[y][x];
        if display_field == '?' {
            println!("if");
            self.display_board[y][x] = 'f';
        } else if display_field == 'f' {
            self.display_board[y][x] = '?';
        }
    }

    fn field_interactive(&self, x: usize, y: usize) -> bool {
        self.display_board[y][x] == '?' || self.display_board[y][x] == 'f'
    }

    fn field_clickable(&self, x: usize, y: usize) -> bool {
        self.display_board[y][x] == '?'
    }
/*
    fn compute_surrounding_mines_board(mines_board_mask: &Vec<Vec<bool>>) -> Vec<Vec<u8>> {
        let mut surrounding_mines_mask: Vec<Vec<u8>> = Vec::new();
        for (y, line) in mines_board_mask.iter().enumerate() {
            let mut line_to_add: Vec<u8> = Vec::new();
            for (x, field) in line.iter().enumerate() {
                let mut bombs: u8 = 0;
                for x_offset in -1i8..2i8 {
                    for y_offset in -1i8..2i8 {
                        let x_check = x as i8 + x_offset;
                        let y_check = y as i8 + y_offset;
                        let above_lower_bounds = x_check >= 0 && y_check >= 0;
                        let below_upper_bounds = line.len() > x_check as usize && mines_board_mask.len() > y_check as usize;
                        let in_bounds = above_lower_bounds && below_upper_bounds;
                        if in_bounds {
                            if mines_board_mask[y_check as usize][x_check as usize] {
                                bombs += 1;
                            }
                        }
                    }
                }
                line_to_add.push(bombs);
            }
            surrounding_mines_mask.push(line_to_add);
        }
        surrounding_mines_mask
    }
*/
    fn compute_surrounding_mines_board(mines_board_mask: &Vec<Vec<bool>>) -> Vec<Vec<u8>> {
        let mut surrounding_mines_board: Vec<Vec<u8>> = Vec::new();
        let height = mines_board_mask.len();
        let width = mines_board_mask[0].len();
        for y in 0..height {
            let mut surrounding_mines_line: Vec<u8> = Vec::new();
            for x in 0..width {
                let mines: u8 = Board::get_field_surrounding_mines_amount(mines_board_mask, x, y);
                surrounding_mines_line.push(mines);
            }
            surrounding_mines_board.push(surrounding_mines_line);
        }
        surrounding_mines_board
    }

    fn get_field_surrounding_mines_amount(mines_board_mask: &Vec<Vec<bool>>, x: usize, y:usize) -> u8 {
        let mut surrounding_mines: u8 = 0;
        let available_coordinates = Board::get_bounded_surrounding_coordinates(x, y, mines_board_mask[0].len(), mines_board_mask.len());
        for coordinate in available_coordinates.iter() {
            let (cx, cy) = coordinate;
            if mines_board_mask[*cy][*cx] {
                surrounding_mines += 1;
            }
        }
        surrounding_mines
    }

    fn generate_mines_board_mask(x: usize, y: usize, bombs: usize) -> Vec<Vec<bool>> {
        let mut board_mask: Vec<Vec<bool>> = vec![vec![false; x]; y];
        let mut bombs_placed = 0;
        let mut rng = thread_rng();
        while bombs_placed < bombs {
            let test_x: usize = rng.gen_range(0, x);
            let test_y: usize = rng.gen_range(0, y);
            if !board_mask[test_y][test_x] {
                board_mask[test_y][test_x] = true;
                bombs_placed += 1;
            }
        }
        return board_mask;
    }

    pub fn has_hit_mine(&self) -> bool { self.hit_mine }

    fn get_unbounded_surrounding_coordinates(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut surrounding: Vec<(usize, usize)> = Vec::new();
        for y_offset in -1i8..2i8 {
            for x_offset in -1i8..2i8 {
                let x_to_visit = x as i8 + x_offset;
                let y_to_visit = y as i8 + y_offset;
                let not_own_field = x_offset != 0 || y_offset != 0;
                if not_own_field {
                    surrounding.push((x_to_visit as usize, y_to_visit as usize));
                }
            }
        }
        surrounding
    }

    fn get_bounded_surrounding_coordinates(x: usize, y: usize, max_exclusive_x: usize, max_exclusive_y: usize) -> Vec<(usize, usize)> {
        let unbounded_surrounding_coordinates = Board::get_unbounded_surrounding_coordinates(x, y);
        let mut bounded_surrounding_coordinates: Vec<(usize, usize)> = Vec::new();
        for coordinate in unbounded_surrounding_coordinates.iter() {
            let (x, y) = coordinate;
            let in_bounds = *x < max_exclusive_x && *y < max_exclusive_y;
            if in_bounds {
                bounded_surrounding_coordinates.push(*coordinate);
            }
        }
        bounded_surrounding_coordinates
    }

    pub fn click_square(&mut self, x: usize, y: usize) {
        let in_bounds = self.mines_board_mask.len() > x && self.mines_board_mask.len() > y;
        if !in_bounds {
            println!("index out of range");
            return;
        } else if !self.field_clickable(x, y) {
            return;
        } else {
            let is_bomb = self.mines_board_mask[y][x];
            if is_bomb {
                self.display_board[y][x] = 'b';
                println!("game over scrub.");
                self.hit_mine = true;
            } else {
                let surrounding_mines = self.surrounding_mines_board[y][x];
                let surrounding_mines_str = surrounding_mines.to_string();
                let mut surrounding_mines_char = surrounding_mines_str.chars().next().unwrap();
                self.display_board[y][x] = surrounding_mines_char;
                if surrounding_mines == 0 {
                    let surrounding_coordinates = Board::get_bounded_surrounding_coordinates(x, y, self.mines_board_mask[0].len(), self.mines_board_mask.len());
                    for coordinate in surrounding_coordinates.iter() {
                        let (vx, vy) = coordinate;
                        self.click_square(*vx, *vy);
                    }
                }
            }
        }
    }

    pub fn print_display_board(&self) {
        for line in self.display_board.iter() {
            let mut l = String::from("");
            for value in line {
                l.push(*value);
                l.push(' ');
                l.push(' ');
            }
            println!("{}", l);
        }
        println!();
    }

    pub fn print_revealed_board(&self) {
        for (y, line) in self.mines_board_mask.iter().enumerate() {
            let mut l = String::from("");
            for (x, value) in line.iter().enumerate() {
                if *value {
                    l.push('b');
                } else {
                    l.push_str(&self.surrounding_mines_board[y][x].to_string());
                }
                l.push(' ');
                l.push(' ');
            }
            println!("{}", l);
        }
    }
}
