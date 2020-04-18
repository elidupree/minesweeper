extern crate ansi_term;
use self::ansi_term::Colour::{Black, Red, White, RGB};
use self::ansi_term::{Colour, Style};

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Contents {
    Mine,
    LosingMine,
    Empty(u8),
}

fn light_blue() -> Colour {
    RGB(0, 34, 255)
}
fn light_green() -> Colour {
    RGB(0, 220, 0)
}
fn turquoise() -> Colour {
    RGB(64, 200, 208)
}

fn brown() -> Colour {
    RGB(165, 42, 42)
}
fn dark_gray() -> Colour {
    RGB(105, 105, 105)
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colors = vec![
            Black.bold(),
            light_blue().bold(),
            light_green().bold(),
            Red.bold(),
            RGB(100, 0, 128).bold(),
            brown().bold(),
            turquoise().bold(),
            RGB(0, 0, 0).bold(),
            dark_gray().bold(),
        ];
        match *self {
            Contents::Mine => write!(f, "{}", Black.paint("☀")),
            Contents::LosingMine => write!(f, "{}", Red.paint("☀")),
            Contents::Empty(0) => write!(f, "{}", Black.paint(" ")),
            Contents::Empty(n) => {
                let s = format!("{}", n);
                write!(f, "{}", colors[n as usize].on(White).paint(s))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SquareState {
    Unguessed,
    Flagged,
    BadFlagged,
    Guessed,
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub contents: Contents,
    pub state: SquareState,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            SquareState::Unguessed => write!(f, "{}", dark_gray().on(White).paint(format!("▧"))),
            SquareState::Guessed => write!(
                f,
                "{}",
                Style::new().on(White).paint(format!("{}", self.contents))
            ),
            SquareState::Flagged => write!(f, "⚑"),
            // TODO: choose diff char
            SquareState::BadFlagged => write!(f, "{}", Red.paint("⚑")),
        }
    }
}
