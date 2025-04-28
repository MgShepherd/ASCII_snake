use rand::Rng;
use std::fmt;

use crate::direction::Direction;
use colored::Colorize;

#[derive(PartialEq, Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    snake_positions: Vec<Position>,
    apple_position: Position,
}

impl Grid {
    //TODO: Handle negative grid sizes
    pub fn new(width: u32, height: u32) -> Self {
        let snake_positions = vec![Position {
            x: width / 2,
            y: height / 2,
        }];

        let apple_position = get_apple_position(width, height, &snake_positions);

        Grid {
            width,
            height,
            snake_positions,
            apple_position,
        }
    }

    pub fn update(&mut self, current_direction: &Direction) -> bool {
        let new_position = self.get_next_snake_pos(current_direction);
        // TODO: Snake collision with itself
        self.check_apple_collision(&new_position);
        self.update_snake_positions(&new_position);
        self.is_snake_location_valid()
    }

    fn get_next_snake_pos(&self, current_direction: &Direction) -> Position {
        match current_direction {
            Direction::Up => Position {
                x: self.snake_positions[0].x,
                y: self.snake_positions[0].y - 1,
            },
            Direction::Down => Position {
                x: self.snake_positions[0].x,
                y: self.snake_positions[0].y + 1,
            },

            Direction::Left => Position {
                x: self.snake_positions[0].x - 1,
                y: self.snake_positions[0].y,
            },

            Direction::Right => Position {
                x: self.snake_positions[0].x + 1,
                y: self.snake_positions[0].y,
            },
        }
    }

    fn check_apple_collision(&mut self, new_position: &Position) {
        if *new_position == self.apple_position {
            self.apple_position =
                get_apple_position(self.width, self.height, &self.snake_positions);
            self.snake_positions.push(Position { x: 0, y: 0 });
        }
    }

    fn is_snake_location_valid(&self) -> bool {
        self.snake_positions[0].x > 0
            && self.snake_positions[0].y > 0
            && self.snake_positions[0].x < self.width - 1
            && self.snake_positions[0].y < self.height - 1
    }

    fn update_snake_positions(&mut self, new_position: &Position) {
        for i in (1..self.snake_positions.len()).rev() {
            self.snake_positions[i] = self.snake_positions[i - 1];
        }
        self.snake_positions[0] = *new_position;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[2J\x1B[1;1H")?;

        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    write!(f, "{}", "#".red())?;
                } else if self.snake_positions.contains(&(Position { x, y })) {
                    write!(f, "{}", "■".blue())?;
                } else if self.apple_position == (Position { x, y }) {
                    write!(f, "{}", "A".green())?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

fn get_apple_position(width: u32, height: u32, snake_positions: &Vec<Position>) -> Position {
    let mut apple_position = Position {
        x: snake_positions[0].x,
        y: snake_positions[0].y,
    };
    while snake_positions.contains(&apple_position) {
        apple_position.x = rand::rng().random_range(1..(width - 1));
        apple_position.y = rand::rng().random_range(1..(height - 1));
    }
    apple_position
}
