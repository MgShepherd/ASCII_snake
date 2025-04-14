use std::fmt;

use crate::input_handler::Direction;

#[derive(PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    snake_position: Position,
}

impl Grid {
    //TODO: Handle negative grid sizes
    pub fn new(width: u32, height: u32) -> Self {
        let starting_position = Position {
            x: width / 2,
            y: height / 2,
        };

        Grid {
            width,
            height,
            snake_position: starting_position,
        }
    }

    pub fn update(&mut self, current_direction: &Direction) -> bool {
        //TODO: Stop snake from being able to go back on itself
        match current_direction {
            Direction::Up => self.snake_position.y -= 1,
            Direction::Down => self.snake_position.y += 1,
            Direction::Left => self.snake_position.x -= 1,
            Direction::Right => self.snake_position.x += 1,
        }
        self.is_snake_location_valid()
    }

    fn is_snake_location_valid(&self) -> bool {
        self.snake_position.x > 0
            && self.snake_position.y > 0
            && self.snake_position.x < self.width - 1
            && self.snake_position.y < self.height - 1
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[2J\x1B[1;1H")?;

        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    write!(f, "#")?;
                } else if self.snake_position == (Position { x, y }) {
                    write!(f, "■")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}
