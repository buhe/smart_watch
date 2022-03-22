use embedded_graphics::prelude::Point;

pub struct Target {
    pub text: String,
    pub point: Point,
}

impl Target {
    pub fn new(text: String,point: Point) -> Self {
        Target {text, point}
    }
}