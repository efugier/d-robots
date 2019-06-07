use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::map::{Acceleration, Angle, Distance, Point, PolyMap, Position, Segment};

/// m/s
const ROBOT_SPEED: Distance = 1.;
const PI: Distance = std::f32::consts::PI;

#[derive(Debug)]
pub enum Event {
    Collision(Point),
    Moved(Distance),
    Turned(Angle),
    Reached(Point),
    Curr(Position),
    Lacc(Vec<Acceleration>),
}
use Event::*;

pub struct Robot {
    // Tx to speak to the app
    app_tx: mpsc::Sender<Event>,
    // Robot position
    pub pos: Position,
    // Actual map used for the simuation
    actual_map: PolyMap,
}

impl Robot {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (app_tx, rx) = mpsc::channel();

        (
            Robot {
                app_tx,
                pos: Position::default(),
                actual_map: PolyMap { polygons: vec![] },
            },
            rx,
        )
    }

    pub fn init(x: Distance, y: Distance, a: Angle) -> (Self, mpsc::Receiver<Event>) {
        let (app_tx, rx) = mpsc::channel();
        (
            Robot {
                app_tx,
                pos: Position {
                    p: Point { x, y },
                    a,
                },
                actual_map: PolyMap { polygons: vec![] },
            },
            rx,
        )
    }

    fn send_to_app(&self, event: Event) {
        self.app_tx.send(event).unwrap();
    }

    // not sure if this deserve its own function
    // TODO: check number of use cases in the near future
    #[cfg(debug_assertions)]
    fn send_to_app_delayed(&self, event: Event, delay: Duration) {
        let tx = self.app_tx.clone();
        thread::spawn(move || {
            thread::sleep(delay);
            tx.send(event).unwrap();
        });
    }

    pub fn go_to(&mut self, dest: &Point) {
        let trajectory = Segment(self.pos.p, *dest);
        let (t, final_pos) = if let Some(stop) = self.actual_map.first_intersection(&trajectory) {
            let t = duration_from_to(self.pos.p, stop);
            (t, stop)
        } else {
            let t = duration_from_to(self.pos.p, *dest);
            (t, *dest)
        };
        self.send_to_app_delayed(Reached(final_pos), t);
        self.pos.p = final_pos;
    }

    pub fn forward(&mut self, dist: Distance) {
        let dest = self.pos.p + Point { x: 0., y: dist }.rotate(self.pos.a);
        self.go_to(&dest);
    }

    pub fn turn(&mut self, angle: Angle) {
        self.pos.a = (self.pos.a + angle) % (2. * PI);
    }

    /// return the last 10 acceleration norms
    pub fn lacc(angle: Angle) {
        unimplemented!()
    }

    /// tune the collision parameters
    /// `nb_acc_for_mean` the number of acceleration norms used to compute a mean
    /// `nb_consec_mean` number of consecutive means to be smaller than `mean_threshold`
    pub fn tune(nb_acc_for_mean: usize, nb_consec_mean: usize, mean_threshold: f32) {
        unimplemented!()
    }

    /// Conditional compilation demo
    pub fn be_a_robot(&self) {
        #[cfg(debug_assertions)]
        {
            let ttl = Duration::from_secs(8);
            println!("I am in debug mode");
            println!("And thus going to kill myself in {:?}", ttl);
            self.send_to_app_delayed(Collision(self.pos.p.clone()), ttl);
        }
        #[cfg(not(debug_assertions))]
        {
            println!("I am in release mode");
        }
    }
}

pub fn duration_from_to(p1: Point, p2: Point) -> Duration {
    let t = p1.sq_dist(&p2).sqrt() / ROBOT_SPEED;
    Duration::from_millis((1000. * t) as u64)
}
