use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_cross_mut};
use imageproc::pixelops::interpolate;
use ndarray::Array2;

use crate::app::AppId;
use crate::map::{Point, Position};

const MAP_WIDTH: u32 = 2;
const MAP_HEIGHT: u32 = 3; // = depth, i.e. dimension in front of the robot
const CENTER_X: f32 = 1.; // position of the 0
const CENTER_Y: f32 = 1.5;
const PIXELS_PER_METER: u32 = 100; // arbitrary precision of 1 cm

#[derive(Debug)]
pub struct AI {
    app_id: AppId,
    all_positions: Vec<Position>,
    collisions: Vec<Point>,
    map_seen: Array2<f32>,
}

/// position in meters
fn pos_to_pixels(point: Point) -> (i32, i32) {
    let Point { x, y } = point;
    let x = ((x + CENTER_X) * PIXELS_PER_METER as f32).round() as i32;
    let y = ((-y + CENTER_Y) * PIXELS_PER_METER as f32).round() as i32;
    (x, y)
}

impl AI {
    pub fn new(app_id: AppId) -> Self {
        AI {
            app_id,
            all_positions: vec![Position::default()],
            collisions: Vec::new(),
            map_seen: Array2::<f32>::zeros((200, 300)), // ça devrait être MAP_X etc mais j'y arrive pas et ça me fait chier
        }
    }
    // demo app interaction
    pub fn be_smart(&mut self, m: &str) -> Option<String> {
        self.all_positions[0].a += 30.;
        self.update_debug_image();
        if m.len() < 4 {
            return None;
        }
        Some(format!("I am smart and can read long sentences"))
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
