use std::{io::stdin, sync::mpsc::Sender};

use termion::{event::Key, input::TermRead};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum SnakeEvent {
    Quit,
    Move(Direction),
}

pub fn handle_input(sender: Sender<SnakeEvent>) {
    for key_event in stdin().keys() {
        match key_event.unwrap() {
            Key::Char('q') => {
                sender.send(SnakeEvent::Quit).unwrap();
                break;
            }
            Key::Char('w') => sender.send(SnakeEvent::Move(Direction::Up)).unwrap(),
            Key::Char('s') => sender.send(SnakeEvent::Move(Direction::Down)).unwrap(),
            Key::Char('a') => sender.send(SnakeEvent::Move(Direction::Left)).unwrap(),
            Key::Char('d') => sender.send(SnakeEvent::Move(Direction::Right)).unwrap(),
            _ => (),
        }
    }
}
