use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

mod polygon;
pub use polygon::Polygon;

/// Approximated zero
const EPSILON: Distance = 1e-6;

/// Centimeters
pub type Distance = f32;

/// Degrees
pub type Angle = f32;

/// Acceleration
pub type Acceleration = f32;

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
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0., y: 0. }
    }

    pub fn angle(&self) -> Angle {
        self.y.atan2(self.x).to_degrees()
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

    pub fn rotate_deg(&self, angle: f32) -> Point {
        let (sin, cos) = angle.to_radians().sin_cos();
        Point {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn sq_norm(&self) -> Distance {
        self.x * self.x + self.y * self.y
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

        // the segments are parallel
        if r_vec_s.abs() < EPSILON {
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
    polygons: Vec<Polygon>,
}

impl PolyMap {
    fn first_intersection(s: &Segment) -> Option<Point> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_test() {
        let p1 = Point { x: 1., y: 2. };
        assert_eq!(Point::zero().angle(), 0.);
        let p2 = Point { x: 1., y: 3. };
        assert_eq!((p2 - p1).angle(), 90.);
        assert_eq!((p1 - p2).angle(), -90.);
    }

    #[test]
    fn intersection_test() {
        let s1 = Segment(Point { x: 0., y: 0. }, Point { x: 2., y: 2. });
        let s2 = Segment(Point { x: 0., y: 2. }, Point { x: 2., y: 0. });

        let intersection = Point { x: 1., y: 1. };

        assert_eq!(Some(intersection), s1.intersection(&s2));

        // Test for parallel
        let s1 = Segment(Point { x: 0., y: 0. }, Point { x: 2., y: 2. });
        let s2 = Segment(Point { x: 0., y: -1. }, Point { x: 2., y: 1. });

        assert_eq!(None, s1.intersection(&s2));

        // Test for t > 1 (intersection not in segment)
        let s1 = Segment(Point { x: 0., y: 0. }, Point { x: 1., y: 1. });
        let s2 = Segment(Point { x: 0., y: 2. }, Point { x: 2., y: 1. });

        assert_eq!(None, s1.intersection(&s2));

        // Test for t < 0 (same but the other way)
        let s1 = Segment(Point { x: 1., y: 1. }, Point { x: 0., y: 0. });
        let s2 = Segment(Point { x: 0., y: 2. }, Point { x: 2., y: 1. });

        assert_eq!(None, s1.intersection(&s2));
    }
}
