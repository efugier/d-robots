use crate::app;
use crate::robot;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

/// Listen asynchronously to different input sources
/// and reduce them to a single rx
pub struct Events {
    rx: mpsc::Receiver<app::Event>,
    // Listen for distant inputs
    _input_file_handle: thread::JoinHandle<()>,
    // Listen for robot input
    _robot_handle: thread::JoinHandle<()>,
    // Listen to past self
    _self_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new(
        input: PathBuf,
        robot_rx: mpsc::Receiver<robot::Event>,
        self_rx: mpsc::Receiver<app::Event>,
    ) -> Events {
        let (tx, rx) = mpsc::channel();

        // listen to the server for distant events
        let _input_file_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let input_file = File::open(&input).expect("Could not open input file");
                loop {
                    let reader = BufReader::new(&input_file);
                    reader.lines().for_each(|line| {
                        tx.send(app::Event::DistantInput(
                            line.expect("Could not read from input file"),
                        ))
                        .unwrap();
                    })
                }
            })
        };

        // listen to the app for user commands
        let _robot_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Ok(event) = robot_rx.recv() {
                    tx.send(app::Event::RobotMessage(event)).unwrap();
                } else {
                    break;
                }
            })
        };

        // listen to the app for user commands
        let _self_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Ok(event) = self_rx.recv() {
                    tx.send(event).unwrap();
                } else {
                    break;
                }
            })
        };

        Events {
            rx,
            _input_file_handle,
            _robot_handle,
            _self_handle,
        }
    }

    pub fn next(&self) -> Result<app::Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
