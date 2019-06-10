use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;

use log;

use crate::ai::AI;
use crate::messages::{Msg, MsgContent::*, MsgId};
use crate::robot::{self, Robot};

pub type AppId = u32;

use crate::events::Events;

pub enum Event {
    /// Robot message
    RobotMessage(robot::Event),
    /// Message from another app
    DistantInput(String),
}

use Event::*;

/// Holds the state of the application
pub struct App {
    // Application id
    pub id: AppId,
    // Contains the intelligence
    ai: AI,
    // Robot interface
    robot: Robot,
    // Output file
    output: File,
    // Tx to send asynchronous message to future-self
    self_tx: mpsc::Sender<Event>,
    // Event handler
    events: Events,
    // Stores the sent messages ids to not rebroadcast them again
    sent_messages_ids: HashSet<MsgId>,
    counter: u32,
}

impl App {
    pub fn new(id: AppId, output: PathBuf, input: PathBuf) -> Self {
        let (self_tx, self_rx) = mpsc::channel();
        let (mut robot, robot_rx) = Robot::new();
        robot.load_map(&"map.json".into());
        let events = Events::new(input, robot_rx, self_rx);
        let output = OpenOptions::new()
            .write(true)
            .append(true)
            .open(output)
            .expect("failed to open output file");

        App {
            id,
            ai: AI::new(id),
            robot,
            output,
            self_tx,
            events,
            sent_messages_ids: HashSet::new(),
            counter: 0,
        }
    }

    fn send_to_network(&mut self, msg: Msg) {
        self.sent_messages_ids.insert(msg.id.clone());

        let msg_str = msg
            .serialize()
            .expect(&format!("Could not serialize the message {:?}", msg));

        if let Err(e) = self.output.write_all(format!("{}\n", msg_str).as_bytes()) {
            log::error!(
                "Failed to write to output file, no one reads from the pipe, {}",
                e
            );
        } else {
            log::info!("sent, messsage: {:?}", msg.content);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.robot.start();
        let greeting_message = Msg::new(
            self.id,
            self.robot.pos.clone(),
            Public(format!("Hello there, I am {}!", self.id)),
        );
        self.send_to_network(greeting_message);

        self.ai.update(&mut self.robot);

        loop {
            // Handle events
            match self.events.next()? {
                RobotMessage(msg) => {
                    log::info!("RobotMessage {:?}", msg);

                    match msg {
                        robot::Event::Reached(p) => {
                            self.ai.update_robot_position(self.id, &p);
                            self.ai.update(&mut self.robot)
                        }
                        robot::Event::Collision(p) => {
                            self.ai.update_robot_position(self.id, &p);
                            self.ai.notify_collision(&mut self.robot, p.p);
                        }
                        _ => break,
                    }

                    self.counter += 1;
                    if self.counter > 10 {
                        self.counter = 0;
                        self.send_to_network(Msg::new(
                            self.id,
                            self.robot.pos.clone(),
                            MapUpdate(self.ai.map_seen.clone()),
                        ));
                    }
                }

                DistantInput(m) => {
                    // log::trace!("message {:?}", m);
                    // AI interaction demo
                    if let Some(msg) = self.ai.be_smart() {
                        println!("{}", msg);
                    }

                    if let Ok(msg) = Msg::from_str(&m) {
                        // do something to the decoded message
                        if !self.sent_messages_ids.contains(&msg.id) {
                            self.ai.update_robot_position(msg.id.clone(), &msg.pos);
                            log::info!("received, from: {} : {:?}", msg.sender_id, msg.content);
                            self.send_to_network(msg.clone());
                        }
                        if let MapUpdate(update) = msg.content {
                            self.ai.merge_maps(update);
                        }
                    } else {
                        log::error!("could not decode {:?}", m);
                    }
                }
            }
        }
        Ok(())
    }
}
