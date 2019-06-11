use std::collections::HashMap;
use std::thread;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_cross_mut, BresenhamLineIter};
use imageproc::pixelops::interpolate;
use itertools::iproduct;
use log;
use ndarray::Array2;
use ndarray::Axis;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::app::AppId;
use crate::map::{Point, Position};
use crate::robot::Robot;

mod pathfinder;

const MAP_WIDTH: u32 = 2;
const MAP_HEIGHT: u32 = 3; // = depth, i.e. dimension in front of the robot
const CENTER_X: f32 = 1.; // position of the 0
const CENTER_Y: f32 = 1.5;
const PIXELS_PER_METER: u32 = 100; // arbitrary precision of 1 cm
const MAP_PWIDTH: usize = (MAP_WIDTH * PIXELS_PER_METER) as usize;
const MAP_PHEIGHT: usize = (MAP_HEIGHT * PIXELS_PER_METER) as usize;
const COLLISION_MERGE_DISTANCE: f32 = 0.1;

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
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

// Removes points where direction does not change from given path
fn smooth_path(path: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    if !path.is_empty() {
        result.push(*path.first().unwrap());
        for i in 1..path.len() - 2 {
            let a = pixels_to_pos(path[i - 1]);
            let b = pixels_to_pos(path[i]);
            let c = pixels_to_pos(path[i + 1]);
            if ((c - a).normalized().dot_prod((b - a).normalized())).abs() < 0.98 {
                result.push(path[i]);
            }
        }
        result.push(*path.last().unwrap());
    }
    result
}

#[derive(Debug)]
pub struct AI {
    app_id: AppId,
    debug_counter: u32,
    all_positions: HashMap<AppId, Position>,
    collisions: Vec<Point>,
    pub map_seen: Array2<CellState>,
    // Next pixel coordinates to go to
    // stored in reverse : last item of Vec is the next point
    next_targets: Vec<(u32, u32)>,
    // Used to mark area as seen between two target points
    next_steps: Vec<(u32, u32)>,
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
            next_targets: Vec::new(),
            next_steps: Vec::new(),
        };
        ai.all_positions.insert(ai.app_id, Position::default());

        ai
    }

    pub fn update(&mut self, robot: &mut Robot) {
        let self_pos = self
            .all_positions
            .get(&self.app_id)
            .expect("self position is missing from all_positions")
            .p;
        while let Some(step) = self.next_steps.pop() {
            // We have reached a target, we need to mark every
            // point from last target to current position as seen
            if pixels_to_pos(step).sq_dist(self_pos) < 0.01 {
                // We have marked every step until current position as seen
                break;
            } else {
                self.mark_seen_circle_at_point(pixels_to_pos(step), 0.1);
            }
        }
        self.mark_seen_circle(0.1);

        if let Some(destination) = self.next_targets.pop() {
            // We still have targets to reach
            log::info!("go_to destination {:?}", destination);
            robot.go_to(pixels_to_pos(destination));
        } else if let Some(target) = self.where_do_we_go() {
            // let delta = (target - self_pos).clip_norm(0.05);
            // log::info!(
            //     "self_pos:{:?} frontier:{:?} goto:{:?}",
            //     self_pos,
            //     target,
            //     delta
            // );
            self.next_steps = pathfinder::find_path(
                pos_to_pixels(self_pos),
                &self.map_seen,
                pos_to_pixels(target),
            );
            self.next_targets = smooth_path(&self.next_steps);

            if let Some(destination) = self.next_targets.pop() {
                log::info!("next moves : {:?}", self.next_targets);
                robot.go_to(pixels_to_pos(destination));
            } else {
                log::error!(
                    "nowhere to go - pathfinding failed. marking target as blocked\nself_pos={:?} {:?}",
                    self_pos, pos_to_pixels(self_pos)
                );
                let (x, y) = pos_to_pixels(target);
                self.map_seen[(x as usize, y as usize)] = Blocked;
            }
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

    pub fn update_robot_position(&mut self, id: AppId, pos: Position) {
        self.all_positions.insert(id, pos);
    }

    pub fn notify_collision(&mut self, robot: &mut Robot, point: Point) {
        self.register_collision(point);
        let (x, y) = pos_to_pixels(point);
        self.map_seen[(x as usize, y as usize)] = Blocked;
        // self.mark_seen_circle(0.1);

        //removing planned path - path will be re-computed
        self.next_steps = Vec::new();
        self.next_targets = Vec::new();

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
        self.mark_seen_circle_at_point(robot, radius);
    }

    fn mark_seen_circle_at_point(&mut self, robot: Point, radius: f32) {
        let (rx, ry) = pos_to_pixels(robot);
        // log::info!("MarkSeen pos={:?} pix={:?}", robot, (rx, ry));
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
            .map(|&p| {
                (
                    p,
                    (p - front).sq_norm()
                        + self
                            .all_positions
                            .iter()
                            .filter(|(&id, _)| id != self.app_id)
                            .map(|(_, &other)| (-(p - other.p).norm()).exp())
                            .sum::<f32>(),
                )
            })
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).expect("NaN here ?"))
            .map(|(p, _)| p)
    }

    /// A frontier is a SeenFree pixel with at least one Uncharted pixel
    /// around it (including diagonal directions).
    /// Note that points are converted back to "real" coordinates, not pixel coordinates.
    fn detect_frontiers(&self) -> Vec<Point> {
        let arr = Self::dilate(&self.map_seen, 2);
        arr.indexed_iter()
            .filter(|(xy, _)| Self::is_frontier(&arr, *xy))
            .map(|((x, y), _)| pixels_to_pos((x as u32, y as u32)))
            .collect()
    }

    fn dilate(arr: &Array2<CellState>, size: usize) -> Array2<CellState> {
        // let prev = arr;
        let mut new = arr.clone();
        // for _ in 0..iterations {
        for xy in iproduct!(size..new.rows() - size, 1..new.cols() - size) {
            if arr[xy] == Uncharted
                && iproduct!(
                    xy.0.saturating_sub(size)..=(xy.0 + size),
                    xy.1.saturating_sub(size)..=(xy.1 + size)
                )
                .any(|n| n != xy && arr[n] == SeenFree)
            {
                new[xy] = SeenFree
            }
        }
        new
    }

    /// Is the xy pixel a frontier ? (Uncharted and has a SeenFree pixel around it)
    fn is_frontier(arr: &Array2<CellState>, xy: (usize, usize)) -> bool {
        arr[xy] == Uncharted
            && iproduct!(
                xy.0.saturating_sub(1)..MAP_PWIDTH.min(xy.0 + 2),
                xy.1.saturating_sub(1)..MAP_PHEIGHT.min(xy.1 + 2)
            )
            .any(|coords| coords != xy && arr[coords] == SeenFree)
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
        self.debug_counter += 1;
        if self.debug_counter % 2 != 0 {
            return;
        }

        let mut img = RgbImage::new(MAP_WIDTH * PIXELS_PER_METER, MAP_HEIGHT * PIXELS_PER_METER);

        for ((x, y), seen) in self.map_seen.indexed_iter() {
            img[(x as u32, y as u32)] = match seen {
                // _ if self.is_frontier((x, y)) => Rgb([0, 200, 0]),
                SeenFree => Rgb([200, 200, 200]),
                Blocked => Rgb([0, 0, 0]),
                Uncharted => Rgb([255, 255, 255]),
            }
        }
        self.detect_frontiers()
            .iter()
            .for_each(|&f| img[pos_to_pixels(f)] = Rgb([0, 200, 0]));

        for (&id, pos) in self.all_positions.iter() {
            let color = if id == self.app_id {
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
        let app_id = self.app_id;
        thread::spawn(move || {
            img.save(temp.clone())
                .unwrap_or_else(|_| panic!("Could not save the debug image for robot {}", app_id));
            std::fs::rename(temp, path)
                .unwrap_or_else(|_| panic!("Could not save the debug image for robot {}", app_id)); // for atomic writes
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
