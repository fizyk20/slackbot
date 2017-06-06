#[macro_use]
extern crate lazy_static;
extern crate slack;
extern crate slack_api;
extern crate regex;
extern crate timer;
extern crate chrono;
extern crate dictionary;
extern crate rand;

mod settings;
mod plugin;
mod plugins;
mod logger;

use logger::Logger;
use plugin::Plugin;
use plugins::*;
use settings::SETTINGS;
use slack::{Event, EventHandler, Message, RtmClient};
use slack_api::MessageStandard;
use std::collections::HashMap;
use std::env;

#[derive(PartialEq, Clone, Copy)]
pub enum ResumeEventHandling {
    Resume,
    Stop,
}

pub enum BotEvent {
    None(ResumeEventHandling),
    Log(String, ResumeEventHandling),
    Send(String, ResumeEventHandling),
}

impl BotEvent {
    pub fn resume_mode(&self) -> ResumeEventHandling {
        match *self {
            BotEvent::None(r) |
            BotEvent::Log(_, r) |
            BotEvent::Send(_, r) => r,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MessageData<'a> {
    pub self_name: &'a str,
    pub user: &'a str,
    pub channel: &'a str,
    pub msg: &'a str,
}

struct BotCore {
    plugins: Vec<Box<Plugin>>,
    users: HashMap<String, String>,
    channels: HashMap<String, String>,
    logger: Logger,
}

impl BotCore {
    fn new() -> BotCore {
        let mut plugins: Vec<Box<Plugin>> = Vec::new();

        // load all used plugins
        plugins.push(Box::new(Patterns::new().unwrap()));
        plugins.push(Box::new(Eightball::new().unwrap()));
        plugins.push(Box::new(RandomChat::new()));

        let log_dir = env::current_dir().unwrap().as_path().join("logs");

        BotCore {
            plugins: plugins,
            users: HashMap::new(),
            channels: HashMap::new(),
            logger: Logger::new(log_dir),
        }
    }

    pub fn handle_message(&mut self, client: &RtmClient, user: &str, channel: &str, msg: &str) {
        let resp = client.start_response();
        let user_name = if let Some(name) = self.users.get(user) {
            &name
        } else {
            user
        };
        let channel_name = if let Some(name) = self.channels.get(channel) {
            &name
        } else {
            channel
        };

        let _ = self.logger.log(format!("<{}> {}", user_name, msg));
        let self_name = resp.slf
            .as_ref()
            .and_then(|u| u.name.as_ref())
            .unwrap();
        let msg_data = MessageData {
            self_name: self_name,
            user: user_name,
            channel: channel_name,
            msg: msg,
        };

        self.plugins
            .sort_by_key(|x| x.plugin_priority(user, channel, msg));

        for plugin in &mut self.plugins {
            // detect if a command
            let command_char = SETTINGS.lock().unwrap().command_char.clone();

            // pass to the plugin
            let result = if msg.starts_with(&command_char) {
                let params = msg[command_char.len()..]
                    .split_whitespace()
                    .map(|x| x.to_lowercase())
                    .collect();
                plugin.handle_command(user_name, channel_name, params)
            } else {
                plugin.handle_message(msg_data)
            };

            // perform the action requested by the plugin
            let resume = result.resume_mode();
            match result {
                BotEvent::Log(message, _) => {
                    let _ = self.logger.log(message);
                }
                BotEvent::Send(message, _) => {
                    let sender = client.sender();
                    if let Err(e) = sender.send_message(channel, &message) {
                        let _ = self.logger
                            .log(format!("***ERROR: Couldn't send message: {:?}", e));
                    } else {
                        let _ = self.logger.log(format!("<{}> {}", self_name, &message));
                    }
                }
                BotEvent::None(_) => (),
            }

            if resume == ResumeEventHandling::Stop {
                break;
            }
        }
    }
}

impl EventHandler for BotCore {
    fn on_event(&mut self, client: &RtmClient, event: Event) {
        match event {
            Event::Message(ref msg) => {
                match **msg {
                    Message::Standard(MessageStandard {
                                          ref user,
                                          ref text,
                                          ref channel,
                                          ..
                                      }) => {
                        let user = user.as_ref().unwrap();
                        let channel = channel.as_ref().unwrap();
                        let text = text.as_ref().unwrap();
                        self.handle_message(client, user, channel, text);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn on_close(&mut self, _: &RtmClient) {
        let _ = self.logger.log("*** Disconnected ***");
    }

    fn on_connect(&mut self, client: &RtmClient) {
        let resp = client.start_response();
        if let Some(ref users) = resp.users {
            for user in users {
                let prefix = if Some(true) == user.is_primary_owner {
                    "&"
                } else if Some(true) == user.is_owner {
                    "~"
                } else if Some(true) == user.is_admin {
                    "@"
                } else {
                    ""
                };
                self.users
                    .insert(user.id.as_ref().cloned().unwrap(),
                            format!("{}{}", prefix, &user.name.as_ref().unwrap()));
            }
        }

        if let Some(ref channels) = resp.channels {
            for channel in channels {
                self.channels
                    .insert(channel.id.as_ref().cloned().unwrap(),
                            channel.name.as_ref().cloned().unwrap());
            }
        }

        let _ = self.logger.log("*** Connected to Slack ***");
    }
}

fn main() {
    let mut handler = BotCore::new();
    // clone to avoid holding the lock
    let token = SETTINGS.lock().unwrap().token.clone();

    if let Err(e) = RtmClient::login_and_run::<BotCore>(&token, &mut handler) {
        println!("{:?}", e);
    }
}
