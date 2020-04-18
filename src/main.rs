extern crate minesweeper_backend;
use minesweeper_backend::backend::{Game, GameState};
use std::io;

fn main() {
    // TODO: add flags for size
    let mut g = Game::new(9, 9, 10);

    while let GameState::InProg = g.state() {
        println!("{}", g);
        println!("What do you want to do? Input: ");
        println!("F|G x y (e.g. F 0 2 to flag (0, 2))");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<&str> = input.trim().split(' ').collect();
        if words.len() != 3 {
            continue;
        }

        let flagged = if words[0] == "F" {
            true
        } else if words[0] == "G" {
            false
        } else {
            println!("Expected F or G (you typed '{}')", words[0]);
            continue;
        };

        let x = match words[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("invalid number (you typed '{}')", words[1]);
                continue;
            }
        };
        let y = match words[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("invalid number (you typed '{}')", words[2]);
                continue;
            }
        };

        if flagged {
            g.toggle_flag(x, y);
        } else {
            g.guess(x, y);
        }
    }
    println!("{}", g);
    match g.state() {
        GameState::Won => {
            println!("You win!");
        }
        GameState::Lost => {
            println!("You lose :(");
        }
        GameState::InProg => {
            panic!("Game went back to inprog after finishing");
        }
    };
}
