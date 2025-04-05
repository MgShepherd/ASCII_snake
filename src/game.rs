use crate::{grid::Grid, renderer::Renderer, snake::Snake};
use std::io::stdin;
use termion::{event::Key, input::TermRead};

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;

pub fn run() {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let mut renderer = Renderer::new();
    let mut running = true;

    renderer.display_grid(&grid, &snake);
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
