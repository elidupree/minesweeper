extern crate ansi_term;
extern crate rand;
use self::rand::distributions::{Range, IndependentSample};
use self::ansi_term::Colour::{Black, White};
use std::fmt;
use square::{Contents, Square, SquareState};
use board_iter::board_iter;

type Board = Vec<Vec<Square>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Won,
    Lost,
    InProg,
}

pub enum PubSquareContents {
    Mine,
    Unguessed,
    Flagged,
    BadFlagged,
    Empty(u8),
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    mines: usize,
    num_guessed: usize,
    generated: bool,
    state: GameState,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print_top(f)?;
        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{} ", i)?;
            for square in row {
                write!(f, "{}", Black.on(White).paint(format!("|{}", square)))?;
            }
            write!(f, "{}", Black.on(White).paint("|"))?;
            write!(f, " {}", i)?;
            write!(f, "\n")?;
        }
        self.print_top(f)?;
        Ok(())
    }
}

impl Game {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Game {
        let mut b = Vec::new();
        for _ in 0..height {
            b.push(vec![Square {
                    contents: Contents::Empty(0),
                    state: SquareState::Unguessed}; width]);
        }
        Game {
            board: b,
            mines: num_mines,
            num_guessed: 0,
            generated: false,
            state: GameState::InProg,
        }
    }

    fn print_top(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  ")?;
        for i in 0..self.board.len() {
            write!(f, "|{}", i)?;
        }
        write!(f, "|\n")
    }

    // Generate a board, with (x, y) not having any surrounding mines.
    fn generate(&mut self, x: usize, y: usize) {
        let mut generated = 0;
        let x_range = Range::new(0, self.board.len());
        let y_range = Range::new(0, self.board[0].len());
        let mut rng = rand::thread_rng();

        while generated < self.mines {
            let mine_x = x_range.ind_sample(&mut rng);
            let mine_y = y_range.ind_sample(&mut rng);
            if let Contents::Mine = self.board[mine_x][mine_y].contents {
                continue;
            }
            if (mine_x as isize - x as isize).abs() > 1 ||
               (mine_y as isize - y as isize).abs() > 1 {
                // Outside the "box" around where the user guessed.
                self.board[mine_x][mine_y].contents = Contents::Mine;
                generated += 1;
                // Increment the mine counts for all surrounding values.
                let iter = board_iter(mine_x, mine_y, self.board.len(), self.board[0].len());
                for (i, j) in iter {
                    if let Contents::Empty(x) = self.board[i][j].contents {
                        self.board[i][j].contents = Contents::Empty(x + 1);
                    }
                }
            }
        }
        self.generated = true;
    }

    pub fn get(&self, x: usize, y: usize) -> PubSquareContents {
        match self.board[x][y].state {
            SquareState::Unguessed => PubSquareContents::Unguessed,
            SquareState::Flagged => PubSquareContents::Flagged,
            SquareState::BadFlagged => PubSquareContents::BadFlagged,
            SquareState::Guessed => {
                match self.board[x][y].contents {
                    Contents::Mine => PubSquareContents::Mine,
                    Contents::LosingMine => PubSquareContents::Mine,
                    Contents::Empty(i) => PubSquareContents::Empty(i),
                }
            }
        }
    }


    // Expand the squares surrounding (x, y)
    fn expand(&mut self, x: usize, y: usize) {
        for (i, j) in board_iter(x, y, self.board.len(), self.board[0].len()) {
            if let GameState::Lost = self.guess_internal(i, j, false) {
                panic!("Expand called on ({}, {}) caused loss", x, y);
            }
        }
    }

    // Return true iff (x, y) is Empty(n) and (x, y) has exactly n flagged
    // squares around it
    fn finished(&mut self, x: usize, y: usize) -> bool {
        if let Contents::Empty(n) = self.board[x][y].contents {
            let mut num_flagged = 0;
            for (i, j) in board_iter(x, y, self.board.len(), self.board[0].len()) {
                if let SquareState::Flagged = self.board[i][j].state {
                    num_flagged += 1;
                }
            }
            num_flagged == n
        } else {
            false
        }
    }

    fn guess_internal(&mut self, x: usize, y: usize, user_direct: bool) -> GameState {
        if !self.generated {
            self.generate(x, y);
        }
        if let GameState::InProg = self.state {
            // Short-circuit to avoid infinite mutual recursion with expand(),
            // and disallow guessing a flagged square.
            match self.board[x][y].state {
                SquareState::Unguessed => {
                    self.board[x][y].state = SquareState::Guessed;
                    if let Contents::Mine = self.board[x][y].contents {
                        // Bail out; you lose!
                        self.state = GameState::Lost;
                        self.board[x][y].contents = Contents::LosingMine;

                        for row in &mut self.board {
                            for sq in row {
                                if let Contents::Mine = sq.contents {
                                    if let SquareState::Unguessed = sq.state {
                                        sq.state = SquareState::Guessed;
                                    }
                                } else if let SquareState::Flagged = sq.state {
                                    // Signal incorrect flag.
                                    sq.state = SquareState::BadFlagged
                                }
                            }
                        }
                    } else {
                        self.num_guessed += 1;
                        if let Contents::Empty(0) = self.board[x][y].contents {
                            self.expand(x, y);
                        }
                        if self.num_guessed + self.mines == self.board.len() * self.board[0].len() {
                            self.state = GameState::Won;
                        }
                    }
                }
                SquareState::Guessed => {
                    if user_direct && self.finished(x, y) {
                        self.expand(x, y);
                    }
                }
                SquareState::Flagged => {}
                SquareState::BadFlagged => {}
            }
        }
        self.state
    }

    pub fn guess(&mut self, x: usize, y: usize) -> GameState {
        self.guess_internal(x, y, true)
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        self.board[x][y].state = match self.board[x][y].state {
            SquareState::Unguessed => SquareState::Flagged,
            SquareState::Flagged => SquareState::Unguessed,
            SquareState::BadFlagged => SquareState::BadFlagged,
            SquareState::Guessed => SquareState::Guessed,
        };
    }

    pub fn state(&self) -> GameState {
        self.state
    }
}
