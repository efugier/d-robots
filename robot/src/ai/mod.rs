use std::collections::HashMap;
use std::thread;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_cross_mut, BresenhamLineIter};
use imageproc::pixelops::interpolate;
use itertools::iproduct;
use log;
use ndarray::Array2;
use ndarray::Axis;
use serde::{Deserialize, Serialize};

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
const COLLISION_MERGE_DISTANCE: f32 = 0.1;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum CellState {
    Uncharted,
    SeenFree,
    Blocked,
}

impl Default for CellState {
    fn default() -> Self {
        Uncharted
    }
}

use CellState::*;

#[derive(Debug)]
pub struct AI {
    app_id: AppId,
    debug_counter: u32,
    all_positions: HashMap<AppId, Position>,
    collisions: Vec<Point>,
    pub map_seen: Array2<CellState>,
}

/// position in meters
fn pos_to_pixels(point: Point) -> (u32, u32) {
    let x = ((point.x + CENTER_X) * PIXELS_PER_METER as f32).round() as u32;
    let y = ((-point.y + CENTER_Y) * PIXELS_PER_METER as f32).round() as u32;
    (x, y)
}

fn pixels_to_pos(p: (u32, u32)) -> Point {
    let x = p.0 as f32 / PIXELS_PER_METER as f32 - CENTER_X;
    let y = -(p.1 as f32 / PIXELS_PER_METER as f32) + CENTER_Y;
    Point { x, y }
}

impl AI {
    pub fn new(app_id: AppId) -> Self {
        let mut ai = AI {
            app_id,
            debug_counter: 0,
            all_positions: HashMap::new(),
            collisions: Vec::new(),
            // the map is uncharted at the start
            map_seen: Array2::<CellState>::default((MAP_PWIDTH, MAP_PHEIGHT)),
        };
        ai.all_positions.insert(ai.app_id, Position::default());
        ai
    }

    pub fn update(&mut self, robot: &mut Robot) {
        self.mark_seen_circle(0.1);

        if let Some(target) = self.where_do_we_go() {
            let self_pos = self
                .all_positions
                .get(&self.app_id)
                .expect("self position is missing from all_positions")
                .p;
            let delta = (target - self_pos).normalized() * 0.05;
            log::info!(
                "self_pos:{:?} frontier:{:?} goto:{:?}",
                self_pos,
                target,
                delta
            );
            robot.go_to(&(self_pos + delta));
        } else {
            log::error!("nowhere to go");
        }

        self.update_debug_image();
    }

    pub fn merge_maps(&mut self, update: Array2<CellState>) {
        for coords in iproduct!(
            0..self.map_seen.len_of(Axis(0)),
            0..self.map_seen.len_of(Axis(1))
        ) {
            let old = self.map_seen.get_mut(coords).unwrap();
            match (*old, update[coords]) {
                (_, Blocked) => *old = Blocked,
                (Uncharted, SeenFree) => *old = SeenFree,
                _ => (),
            }
        }
    }

    // demo app interaction
    pub fn be_smart(&mut self) -> Option<String> {
        self.mark_seen_circle(0.1);
        self.update_debug_image();
        Some(String::from("Ok"))
    }

    pub fn update_robot_position(&mut self, id: AppId, pos: &Position) {
        self.all_positions.insert(id, pos.clone());
    }

    pub fn notify_collision(&mut self, robot: &mut Robot, point: Point) {
        self.register_collision(point);
        let (x, y) = pos_to_pixels(point);
        self.map_seen[(x as usize, y as usize)] = Blocked;
        // self.mark_seen_circle(0.1);

        robot.forward(-0.1);
        self.update_debug_image();
    }

    fn register_collision(&mut self, new: Point) {
        let start = pos_to_pixels(new);
        let s_f32 = (start.0 as f32, start.1 as f32);
        for &p in self.collisions.iter() {
            if (p - new).sq_norm() <= COLLISION_MERGE_DISTANCE * COLLISION_MERGE_DISTANCE {
                let end = pos_to_pixels(p);
                let e_f32 = (end.0 as f32, end.1 as f32);
                for x in BresenhamLineIter::new(s_f32, e_f32) {
                    self.map_seen[(x.0 as usize, x.1 as usize)] = Blocked;
                }
            }
        }
        self.collisions.push(new);
    }

    /// mark the area around the robot as seen (in a circle, radius in meters)
    fn mark_seen_circle(&mut self, radius: f32) {
        let robot = self
            .all_positions
            .get(&self.app_id)
            .expect("self position is missing from all_positions")
            .p;
        let (rx, ry) = pos_to_pixels(robot);
        log::info!("MarkSeen pos={:?} pix={:?}", robot, (rx, ry));
        let radius_p = (radius * PIXELS_PER_METER as f32).ceil() as u32;
        for y in ry.saturating_sub(radius_p)..(ry + radius_p + 1).min(MAP_PHEIGHT as u32) {
            for x in rx.saturating_sub(radius_p)..(rx + radius_p + 1).min(MAP_PWIDTH as u32) {
                let xy = (x, y);
                let dist = (pixels_to_pos(xy) - robot).sq_norm();
                let ixy = (x as usize, y as usize);
                if dist <= radius * radius && self.map_seen[ixy] == Uncharted {
                    self.map_seen[ixy] = SeenFree;
                }
            }
        }
    }

    /// https://www.youtube.com/watch?v=1w7OgIMMRc4
    /// If not frontier is detected, go back to the origin
    fn where_do_we_go(&self) -> Option<Point> {
        let pos = &self
            .all_positions
            .get(&self.app_id)
            .expect("self position is missing from all_positions");
        // point a little bit in front of the robot, because i want to prioritise frontier points in front of the robot
        let front = pos.p + Point { x: 0., y: 0.05 }.rotate(pos.a);
        self.detect_frontiers()
            .iter()
            .map(|p| (p, (*p - front).sq_norm()))
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).expect("NaN here ?"))
            .map(|(p, _)| *p)
        // .unwrap_or(&Point::zero())
    }

    /// A frontier is a SeenFree pixel with at least one Uncharted pixel
    /// around it (including diagonal directions).
    /// Note that points are converted back to "real" coordinates, not pixel coordinates.
    fn detect_frontiers(&self) -> Vec<Point> {
        self.map_seen
            .indexed_iter()
            .filter(|(xy, _)| self.is_frontier(*xy))
            .map(|((x, y), _)| pixels_to_pos((x as u32, y as u32)))
            .collect()
    }

    /// Is the xy pixel a frontier ? (SeenFree and has an Uncharted pixel around it)
    fn is_frontier(&self, xy: (usize, usize)) -> bool {
        self.map_seen[xy] == SeenFree
            && iproduct!(
                xy.0.saturating_sub(1)..MAP_PWIDTH.min(xy.0 + 2),
                xy.1.saturating_sub(1)..MAP_PHEIGHT.min(xy.1 + 2)
            )
            .any(|coords| coords != xy && self.map_seen[coords] == Uncharted)
    }

    fn draw_robot(&self, img: &mut RgbImage, pos: &Position, color: Rgb<u8>) {
        let (x, y) = pos_to_pixels(pos.p);
        let end = pos_to_pixels((Point { x: 0., y: 0.05 }).rotate(pos.a) + pos.p);
        draw_cross_mut(img, color, x as i32, y as i32);
        draw_antialiased_line_segment_mut(
            img,
            (x as i32, y as i32),
            (end.0 as i32, end.1 as i32),
            color,
            interpolate,
        );
    }

    fn update_debug_image(&mut self) {
        let mut img = RgbImage::new(MAP_WIDTH * PIXELS_PER_METER, MAP_HEIGHT * PIXELS_PER_METER);

        for ((x, y), seen) in self.map_seen.indexed_iter() {
            img[(x as u32, y as u32)] = match seen {
                _ if self.is_frontier((x, y)) => Rgb([0, 200, 0]),
                SeenFree => Rgb([200, 200, 200]),
                Blocked => Rgb([0, 0, 0]),
                Uncharted => Rgb([255, 255, 255]),
            }
        }

        for (i, pos) in self.all_positions.values().enumerate() {
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
        // let path = format!("output/robot_{}-{}.png", self.app_id, self.debug_counter);
        let temp = format!(
            "output/robot_{}-{}_tmp.png",
            self.app_id, self.debug_counter
        );
        self.debug_counter += 1;
        let app_id = self.app_id;
        thread::spawn(move || {
            img.save(temp.clone()).expect(&format!(
                "Could not save the debug image for robot {}",
                app_id
            ));
            std::fs::rename(temp, path).expect(&format!(
                "Could not save the debug image for robot {}",
                app_id
            )); // for atomic writes
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixels_pos_test() {
        let pix = (MAP_PWIDTH as u32 / 2, MAP_PHEIGHT as u32 / 2);
        assert_eq!(pixels_to_pos(pix), Point::zero());
        assert_eq!(pos_to_pixels(Point::zero()), pix);

        let a = (pix.0, pix.0 + 10);
        eprintln!("a={:?} pos={:?}", a, pixels_to_pos(a));
        assert_eq!(pos_to_pixels(pixels_to_pos(a)), a);

        let b = (pix.0 + 10, pix.0 - 5);
        println!("a={:?} pos={:?}", b, pixels_to_pos(b));
        assert_eq!(pos_to_pixels(pixels_to_pos(b)), b);
    }
}
