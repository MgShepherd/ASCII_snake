use crate::{grid::Grid, snake::Snake};
use std::io::{stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;

pub fn run() {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let mut running = true;

    let mut _stdout = stdout().into_raw_mode().unwrap();

    println!("{}", grid);
    while running {
        handle_input(&mut snake, &mut running);
    }
}

fn handle_input(snake: &mut Snake, running: &mut bool) {
    for key_event in stdin().keys() {
        match key_event.unwrap() {
            Key::Char('w') => {
                snake.y -= 1;
            }
            Key::Char('q') => {
                *running = false;
                break;
            }
            _ => (),
        }
    }
}
