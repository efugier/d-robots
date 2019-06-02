use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::map::{Acceleration, Angle, Distance, Point, Position};

#[derive(Debug)]
pub enum Event {
    Collision(Point),
    Moved(Distance),
    Turned(Angle),
    Reached(Distance, Distance),
    Curr(Position),
    Lacc(Vec<Acceleration>),
}
use Event::*;

pub struct Robot {
    // Tx to speak to the app
    app_tx: mpsc::Sender<Event>,
    // Robot position
    pub pos: Position,
}

impl Robot {
    pub fn new() -> (Self, mpsc::Receiver<Event>) {
        let (app_tx, rx) = mpsc::channel();

        (
            Robot {
                app_tx,
                pos: Position::default(),
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
    pub fn go_to(x: Distance, y: Distance) {
        unimplemented!()
    }
    pub fn forward(dist: Distance) {
        unimplemented!()
    }
    pub fn turn(angle: Angle) {
        unimplemented!()
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
