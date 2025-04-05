pub struct Grid {
    pub width: u32,
    pub height: u32,
}

impl Grid {
    //TODO: Handle negative grid sizes
    pub fn new(width: u32, height: u32) -> Self {
        Grid { width, height }
    }
}
