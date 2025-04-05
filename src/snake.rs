pub struct Snake {
    pub x: u32,
    pub y: u32,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
