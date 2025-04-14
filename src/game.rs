use crate::{
    grid::Grid,
    input_handler::{self, Direction, SnakeEvent},
    snake::Snake,
};
use std::{
    io::stdout,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, SystemTime},
};
use termion::raw::IntoRawMode;

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;

const FPS: u32 = 10;

pub fn run() {
    let (sender, reciever) = mpsc::channel::<SnakeEvent>();
    let _stdout = stdout().into_raw_mode().unwrap();

    let input_handler = thread::spawn(|| input_handler::handle_input(sender));

    game_loop(&reciever);
    input_handler.join().unwrap();
}

fn game_loop(reciever: &Receiver<SnakeEvent>) {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let target_time = 1000 / FPS as u128;

    println!("Press the Q key to exit the game\r");
    let mut i = 0;
    loop {
        let start = SystemTime::now();
        if process_input(reciever, &mut snake) {
            break;
        }
        println!("{}", grid);
        println!("Iteration: {i}\r");
        i += 1;
        let elapsed = start.elapsed().unwrap();
        thread::sleep(Duration::from_millis(
            (target_time - elapsed.as_millis()) as u64,
        ));
    }
}

fn process_input(reciever: &Receiver<SnakeEvent>, snake: &mut Snake) -> bool {
    for recieved in reciever.try_iter() {
        match recieved {
            SnakeEvent::Quit => return true,
            SnakeEvent::Move(direction) => match direction {
                Direction::Up => snake.y -= 1,
                Direction::Down => snake.y += 1,
                Direction::Left => snake.x -= 1,
                Direction::Right => snake.x += 1,
            },
        }
    }
    false
}
