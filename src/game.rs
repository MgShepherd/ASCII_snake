use crate::{
    direction::Direction,
    grid::Grid,
    input_handler::{self, SnakeEvent},
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

    thread::spawn(|| input_handler::handle_input(sender));

    game_loop(&reciever);
}

fn game_loop(reciever: &Receiver<SnakeEvent>) {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let target_time = 1000 / FPS as u128;
    let mut current_direction = Direction::Up;

    println!("Press the Q key to exit the game\r");
    let mut i = 0;
    loop {
        let start = SystemTime::now();
        if process_input(reciever, &mut current_direction) {
            break;
        }
        if !grid.update(&current_direction) {
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
    println!("Game has finished\r");
}

fn process_input(reciever: &Receiver<SnakeEvent>, current_direction: &mut Direction) -> bool {
    for recieved in reciever.try_iter() {
        match recieved {
            SnakeEvent::Quit => return true,
            SnakeEvent::Move(direction) => {
                if current_direction.is_next_direction_valid(&direction) {
                    *current_direction = direction;
                }
            }
        }
    }
    false
}
