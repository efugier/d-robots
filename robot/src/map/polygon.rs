use super::{Point, Position};

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn intersection(&self, pos: Position) -> Option<Point> {
        unimplemented!();
    }
}
