use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_cross_mut};
use imageproc::pixelops::interpolate;
use ndarray::Array2;

use itertools::iproduct;

use crate::app::AppId;
use crate::map::{Point, Position};
use crate::robot::Robot;

const MAP_WIDTH: u32 = 2;
const MAP_HEIGHT: u32 = 3; // = depth, i.e. dimension in front of the robot
const CENTER_X: f32 = 1.; // position of the 0
const CENTER_Y: f32 = 1.5;
const PIXELS_PER_METER: u32 = 100; // arbitrary precision of 1 cm
const MAP_PWIDTH: usize = (MAP_WIDTH * PIXELS_PER_METER) as usize;
const MAP_PHEIGHT: usize = (MAP_HEIGHT * PIXELS_PER_METER) as usize;

const UNCHARTED: i8 = 0;
const SEEN_FREE: i8 = 1;
const BLOCKED: i8 = -1;

#[derive(Debug)]
pub struct AI {
    app_id: AppId,
    all_positions: Vec<Position>,
    collisions: Vec<Point>,
    map_seen: Array2<i8>,
}

/// position in meters
fn pos_to_pixels(point: Point) -> (i32, i32) {
    let Point { x, y } = point;
    let x = ((x + CENTER_X) * PIXELS_PER_METER as f32).round() as i32;
    let y = ((-y + CENTER_Y) * PIXELS_PER_METER as f32).round() as i32;
    (x, y)
}

fn pixels_to_pos(p: (i32, i32)) -> Point {
    let x = p.0 as f32 / PIXELS_PER_METER as f32 - CENTER_X;
    let y = p.1 as f32 / PIXELS_PER_METER as f32 - CENTER_Y;
    Point { x, y }
}

impl AI {
    pub fn new(app_id: AppId) -> Self {
        AI {
            app_id,
            all_positions: vec![Position::default()],
            collisions: Vec::new(),
            map_seen: Array2::<i8>::zeros((MAP_PWIDTH, MAP_PHEIGHT)),
        }
    }

    pub fn update(&mut self, robot: &mut Robot) {
        self.mark_seen_circle(0.1);

        if let Some(p) = self.where_do_we_go() {
            let delta = (p - self.all_positions[0].p).normalized() * 0.1;
            robot.go_to(&(self.all_positions[0].p + delta));
        } else {
            log::error!("nowhere to go");
        }
    }

    // demo app interaction
    pub fn be_smart(&mut self, m: &str) -> Option<String> {
        self.mark_seen_circle(0.1);

        println!("frontiers {:?}", self.detect_frontiers());

        self.all_positions[0].a += 30.;
        self.update_debug_image();
        Some(String::from("Ok"))
    }

    /// mark the area around the robot as seen (in a circle, radius in meters)
    fn mark_seen_circle(&mut self, radius: f32) {
        let robot = self.all_positions[0].p;
        let (rx, ry) = pos_to_pixels(robot);
        let radius_p = (radius * PIXELS_PER_METER as f32).ceil() as i32;
        for y in -radius_p..=radius_p {
            let iy = ry + y;
            if iy < 0 || iy >= self.map_seen.rows() as i32 {
                continue;
            }

            for x in -radius_p..=radius_p {
                let ix = rx + x;
                if ix < 0 || ix >= self.map_seen.cols() as i32 {
                    continue;
                }

                let dist = (pixels_to_pos((ix, iy)) - robot).sq_norm();
                // eprintln!("i'm at ix:{} iy:{} dist is {}", ix, iy, dist);
                if dist <= radius * radius {
                    self.map_seen[(ix as usize, iy as usize)] = SEEN_FREE;
                }
            }
        }
    }

    /// https://www.youtube.com/watch?v=1w7OgIMMRc4
    /// If not frontier is detected, go back to the origin
    fn where_do_we_go(&self) -> Option<Point> {
        let pos = &self.all_positions[0];
        // point a little bit in front of the robot, because i want to prioritise frontier points in front of the robot
        let front = pos.p + Point { x: 0., y: 0.1 }.rotate_deg(pos.a);
        self.detect_frontiers()
            .iter()
            .map(|p| (p, (*p - front).sq_norm()))
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).expect("NaN here ?"))
            .map(|(p, _)| *p)
        // .unwrap_or(&Point::zero())
    }

    /// A frontier is a SEEN_FREE pixel with at least one UNCHARTED pixel
    /// around it (including diagonal directions).
    /// Note that points are converted back to "real" coordinates, not pixel coordinates.
    fn detect_frontiers(&self) -> Vec<Point> {
        self.map_seen
            .indexed_iter()
            .filter(|(xy, _)| self.is_frontier(*xy))
            .map(|((x, y), _)| pixels_to_pos((x as i32, y as i32)))
            .collect()
    }

    /// Is the xy pixel a frontier ? (SEEN_FREE and has a UNCHARTED pixel around it)
    fn is_frontier(&self, xy: (usize, usize)) -> bool {
        if self.map_seen[xy] != SEEN_FREE {
            return false;
        }
        iproduct!(
            xy.0.saturating_sub(1)..MAP_PWIDTH.min(xy.0 + 2),
            xy.1.saturating_sub(1)..MAP_PHEIGHT.min(xy.1 + 2)
        )
        .any(|coords| coords != xy && self.map_seen[coords] == UNCHARTED)
    }

    fn draw_robot(&self, img: &mut RgbImage, pos: &Position, color: Rgb<u8>) {
        let (x, y) = pos_to_pixels(pos.p);
        let end = pos_to_pixels((Point { x: 0., y: 0.1 }).rotate_deg(pos.a) + pos.p);
        draw_cross_mut(img, color, x, y);
        draw_antialiased_line_segment_mut(img, (x, y), end, color, interpolate);
    }

    fn update_debug_image(&self) {
        let mut img = RgbImage::from_pixel(
            MAP_WIDTH * PIXELS_PER_METER,
            MAP_HEIGHT * PIXELS_PER_METER,
            Rgb([255, 255, 255]),
        );

        for ((x, y), seen) in self.map_seen.indexed_iter() {
            if self.is_frontier((x, y)) {
                img[(x as u32, y as u32)] = Rgb([0, 200, 0]);
            } else if *seen == SEEN_FREE {
                img[(x as u32, y as u32)] = Rgb([200, 200, 200]);
            } else if *seen == BLOCKED {
                img[(x as u32, y as u32)] = Rgb([0, 0, 0]);
            }
        }

        for (i, pos) in self.all_positions.iter().enumerate() {
            let color = if i == 0 {
                Rgb([255, 0, 0])
            } else {
                Rgb([0, 0, 255])
            };
            self.draw_robot(&mut img, pos, color);
        }

        std::fs::create_dir_all("output")
            .expect("Could not create the directory for outputing debug images");
        let path = format!("output/robot_{}.png", self.app_id);
        img.save(path).expect(&format!(
            "Could not save the debug image for robot {}",
            self.app_id
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixels_pos_test() {
        let pix = (MAP_PWIDTH as i32 / 2, MAP_PHEIGHT as i32 / 2);
        assert_eq!(pixels_to_pos(pix), Point::zero());
        assert_eq!(pos_to_pixels(Point::zero()), pix);
    }
}
