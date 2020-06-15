#[derive(Debug)]
pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl BoundingBox {
    pub fn width(&self) -> usize {
        (self.max_x - self.min_x) as usize
    }

    pub fn height(&self) -> usize {
        (self.max_y - self.min_y) as usize
    }

    pub fn area(&self) -> i64 {
        self.width() as i64 * self.height() as i64
    }
}
