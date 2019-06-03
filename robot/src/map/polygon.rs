use super::{Point, Position, Segment};

pub struct Polygon {
    pub points: Vec<Point>,
    pub is_closed: bool,
}

impl Polygon {
    pub fn segments<'a>(&'a self) -> IterPolygon<'a> {
        IterPolygon {
            polygon: self,
            curr: 0,
            next: 1,
        }
    }
}

pub struct IterPolygon<'a> {
    polygon: &'a Polygon,
    curr: usize,
    next: usize,
}

impl<'a> Iterator for IterPolygon<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.polygon.points.len() {
            None
        } else {
            let seg = Segment(
                self.polygon.points[self.curr],
                self.polygon.points[self.next],
            );
            self.curr += 1;
            self.next += 1;
            if self.polygon.is_closed && self.next >= self.polygon.points.len() {
                self.next = 0;
            }
            Some(seg)
        }
    }
}
