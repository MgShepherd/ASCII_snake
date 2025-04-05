use std::io::{Stdout, Write, stdout};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::{grid::Grid, snake::Snake};

pub struct Renderer {
    stdout: RawTerminal<Stdout>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn display_grid(&mut self, grid: &Grid, snake: &Snake) {
        write!(self.stdout, "\x1B[2J\x1B[1;1H").unwrap();

        for y in 0..grid.height {
            for x in 0..grid.width {
                if y == 0 || y == grid.height - 1 || x == 0 || x == grid.width - 1 {
                    write!(self.stdout, "#").unwrap();
                } else if x == snake.x && y == snake.y {
                    write!(self.stdout, "■").unwrap();
                } else {
                    write!(self.stdout, " ").unwrap();
                }
            }
            write!(self.stdout, "\r\n").unwrap();
        }
    }
}
