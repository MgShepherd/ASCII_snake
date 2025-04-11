use crate::{
    grid::Grid,
    input_handler::{self, Direction, SnakeEvent},
    snake::Snake,
};
use std::{
    io::stdout,
    sync::mpsc::{self, Receiver},
    thread,
};
use termion::raw::IntoRawMode;

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;

pub fn run() {
    let _grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);

    let (sender, reciever) = mpsc::channel::<SnakeEvent>();
    let _stdout = stdout().into_raw_mode().unwrap();

    let input_handler = thread::spawn(|| input_handler::handle_input(sender));

    println!("Press the Q key to exit the game\r");
    loop {
        if process_input(&reciever, &mut snake) {
            break;
        }
    }

    input_handler.join().unwrap();
}

fn process_input(reciever: &Receiver<SnakeEvent>, snake: &mut Snake) -> bool {
    match reciever.try_recv() {
        Ok(snake_event) => match snake_event {
            SnakeEvent::Quit => return true,
            SnakeEvent::Move(direction) => match direction {
                Direction::Up => snake.y -= 1,
                Direction::Down => snake.y += 1,
                Direction::Left => snake.x -= 1,
                Direction::Right => snake.x += 1,
            },
        },
        _ => {}
    }
    false
}
