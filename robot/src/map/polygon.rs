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
        let len = self.polygon.points.len();
        if self.curr >= len {
            return None;
        }

        // last item
        // a polygon with a single point is always considered closed
        if self.next >= len && (self.polygon.is_closed || len == 1) {
            self.next = 0;
        }

        let seg = Segment(
            self.polygon.points[self.curr],
            self.polygon.points[self.next],
        );
        self.curr += 1;
        self.next += 1;
        Some(seg)
    }
}
