use super::{Point, Segment};

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
        if (self.curr >= len - 1 && !self.polygon.is_closed) || self.curr >= len {
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
