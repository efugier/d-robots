use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

mod polygon;
pub use polygon::Polygon;

/// Approximated zero
const ZERO: Distance = 1e-6;
/// Centimeters
pub type Distance = f32;

/// Degrees
pub type Angle = f32;

/// Acceleration
pub type Acc = f32;

#[derive(Copy, Default, Clone, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: Distance,
    pub y: Distance,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Point> for Distance {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < ZERO && (self.y - other.y).abs() < ZERO
    }
}

impl Point {
    pub fn angle(&self, other: &Point) -> Angle {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        to_degrees((dy / dx).atan())
    }

    pub fn vec_to(&self, other: &Point) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn dot_prod(&self, other: &Point) -> Distance {
        self.x * other.x + self.y * other.y
    }

    pub fn cross_prod(&self, other: &Point) -> Distance {
        self.x * other.y - self.y * other.x
    }
}

pub struct Segment(Point, Point);

impl Segment {
    /// seg1 = p + r
    /// seg2 = q + s
    /// intersection <=> exists t,u in [0, 1] such that
    /// p + tr = q + us
    /// where t = (q - p).cross_prod(s) / (r.cross_prod(s))
    pub fn intersection(&self, other: &Segment) -> Option<Point> {
        // p = self.0
        // q = other.0
        let r = self.0.vec_to(&self.1);
        let s = other.0.vec_to(&other.1);

        let r_vec_s = r.cross_prod(&s);

        // the segments are parrallel
        if r_vec_s.abs() < ZERO {
            return None;
        }

        let t = (other.0 - self.0).cross_prod(&s) / r_vec_s;

        // the segment does not intersect
        if t < 0. || t > 1. {
            None
        } else {
            Some(self.0 + t * r)
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Position {
    pub p: Point,
    pub a: Angle,
}

pub struct PolyMap {
    ploygons: Vec<Polygon>,
}

impl PolyMap {}

fn to_degrees(r: Angle) -> Angle {
    r * 180.0 / std::f32::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_test() {
        let s1 = Segment(Point { x: 0., y: 0. }, Point { x: 2., y: 2. });
        let s2 = Segment(Point { x: 0., y: 2. }, Point { x: 2., y: 0. });

        let intersection = Point { x: 1., y: 1. };

        assert_eq!(Some(intersection), s1.intersection(&s2));

        let s1 = Segment(Point { x: 0., y: 0. }, Point { x: 1., y: 1. });
        let s2 = Segment(Point { x: 0., y: 2. }, Point { x: 2., y: 1. });

        assert_eq!(None, s1.intersection(&s2));
    }
}
