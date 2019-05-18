use crate::app::Position;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum Event {
    Collision,
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
                pos: Position { x: 0.0, y: 0.0 },
            },
            rx,
        )
    }
    fn send_to_app(&self, event: Event) {
        self.app_tx.send(event).unwrap();
    }
    // not sure if this deserve its own function
    // check number of use cases
    #[cfg(debug_assertions)]
    fn send_to_app_delayed(&self, event: Event, delay: Duration) {
        let tx = self.app_tx.clone();
        thread::spawn(move || {
            thread::sleep(delay);
            tx.send(event).unwrap();
        });
    }
    /// Conditional compilation demo
    pub fn be_a_robot(&self) {
        #[cfg(debug_assertions)]
        {
            let ttl = Duration::from_secs(8);
            println!("I am in debug mode");
            println!("And thus going to kill myself in {:?}", ttl);
            self.send_to_app_delayed(Collision, ttl);
        }
        #[cfg(not(debug_assertions))]
        {
            println!("I am in release mode");
        }
    }
}
