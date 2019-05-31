use serde::{Deserialize, Serialize};
mod polygon;
pub use polygon::Ploygon;

/// Centimeters
pub type Distance = f32;

/// Degrees
pub type Angle = f32;

/// Acceleration
pub type Acc = f32;

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Point {
    pub x: Distance,
    pub y: Distance,
}

impl Point {
    pub fn angle(&self, p: &Point) -> Angle {
        let dx = self.x - p.x;
        let dy = self.y - p.y;

        to_degrees((dy / dx).atan())
    }
}

pub struct Segment(Point, Point);

impl Segment {
    pub fn intersection(&self, pos: &Position) -> Option<Point> {
        let p1;
        let p2;
        if self.0.x <= self.1.x {
            p1 = &self.0;
            p2 = &self.1;
        } else {
            p1 = &self.1;
            p2 = &self.0;
        };

        // intersection exists
        if pos.p.angle(p1) <= pos.a && pos.a <= pos.p.angle(p2) {
            return Some(Point::default());
        }
        None
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    pub p: Point,
    pub a: Angle,
}

impl Position {}

pub struct PolyMap {
    ploygons: Vec<Ploygon>,
}

impl PolyMap {}

fn to_degrees(r: Angle) -> Angle {
    r * 180.0 / std::f32::consts::PI
}
