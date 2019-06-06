use super::{Point, Segment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Polygon {
    pub points: Vec<Point>,
    pub is_closed: bool,
}

impl Polygon {
    pub fn segments<'a>(&'a self) -> IterPolygon<'a> {
        IterPolygon {
            polygon: self,
            curr: 0,
        }
    }
}

pub struct IterPolygon<'a> {
    polygon: &'a Polygon,
    curr: usize,
}

impl<'a> Iterator for IterPolygon<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.polygon.points.len();
        // case len = 1, and 2 are special
        if (self.curr >= len - 1 && (!self.polygon.is_closed || len < 3)) || self.curr >= len {
            None
        } else {
            let seg = Segment(
                self.polygon.points[self.curr],
                self.polygon.points[(self.curr + 1) % len],
            );
            self.curr += 1;
            Some(seg)
        }
    }
}
