use crate::snake::Snake;

pub struct Grid {
    width: u32,
    height: u32,
}

impl Grid {
    //TODO: Handle negative grid sizes
    pub fn new(width: u32, height: u32) -> Self {
        Grid { width, height }
    }

    pub fn display(&self, snake: &Snake) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    print!("#");
                } else if x == snake.x && y == snake.y {
                    print!("■");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
