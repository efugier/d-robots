use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;

use log;

use crate::ai::AI;
use crate::messages::{Header::*, Msg, MsgId};
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
}

impl App {
    pub fn new(id: AppId, output: PathBuf, input: PathBuf) -> Self {
        let (self_tx, self_rx) = mpsc::channel();
        let (robot, robot_rx) = Robot::new();
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
            log::info!("sent, messsage: {:?}", msg.header);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.robot.be_a_robot();
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
                // Robot message demo
                RobotMessage(e) => {
                    log::info!("boom {:?}", e);
                    break;
                }

                DistantInput(m) => {
                    log::trace!("message {:?}", m);
                    // AI interaction demo
                    if let Some(msg) = self.ai.be_smart() {
                        println!("{}", msg);
                    }

                    if let Ok(msg) = Msg::from_str(&m) {
                        // do something to the decoded message
                        if !self.sent_messages_ids.contains(&msg.id) {
                            self.ai
                                .update_robot_position(msg.id.clone(), msg.pos.clone());
                            log::info!("received, from: {} : {:?}", msg.sender_id, msg.header);
                            self.send_to_network(msg.clone());
                        }
                        if let MapUpdate(update) = msg.header {
                            self.ai.update_map(update);
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
