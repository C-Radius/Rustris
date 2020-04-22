use ggez::graphics::Color;

#[derive(Copy, Clone, Debug)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x: x, y: y }
    }
}

impl Point2D<i32> {
    pub fn rotate(&mut self, deg: f32) {
        self.x = (self.x as f32 * deg.cos() - self.y as f32 * deg.sin()) as i32;
        self.y = (self.y as f32 * deg.cos() + self.x as f32 * deg.sin()) as i32;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub position: Point2D<i32>,
    pub color: Color,
}

impl Block {
    pub fn rotate(&mut self, deg: f32) {
        self.position.rotate(deg);
    }
}
