pub mod grid;
pub mod snake;

use crate::grid::Grid;
use crate::snake::Snake;

const GRID_WIDTH: u32 = 30;
const GRID_HEIGHT: u32 = 20;

fn main() {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let snake = Snake::new(GRID_WIDTH / 2, GRID_HEIGHT / 2);

    grid.display(&snake);
}
