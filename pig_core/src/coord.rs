pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}