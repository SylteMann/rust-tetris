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
