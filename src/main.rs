#[macro_use]
extern crate lazy_static;
extern crate slack;
extern crate regex;
extern crate timer;
extern crate chrono;
extern crate dictionary;
extern crate rand;

mod settings;
mod plugin;
mod plugins;
mod logger;

use std::collections::HashMap;
use std::env;
use slack::{RtmClient, EventHandler, Event, Error, Message};
use settings::SETTINGS;
use logger::{Logger, LogMode};
use plugin::Plugin;
use plugins::*;

#[derive(PartialEq, Clone, Copy)]
pub enum ResumeEventHandling {
    Resume,
    Stop
}

pub enum BotEvent {
    None(ResumeEventHandling),
    Log(String, ResumeEventHandling),
    Send(String, ResumeEventHandling)
}

impl BotEvent {
    pub fn resume_mode(&self) -> ResumeEventHandling {
        match *self {
            BotEvent::None(r) => r,
            BotEvent::Log(_, r) => r,
            BotEvent::Send(_, r) => r
        }
    }
}

struct BotCore {
    plugins: Vec<Box<Plugin>>,
    users: HashMap<String, String>,
    channels: HashMap<String, String>,
    logger: Logger
}

impl BotCore {

    fn new() -> BotCore {
        let mut plugins: Vec<Box<Plugin>> = Vec::new();

        // load all used plugins
        plugins.push(Box::new(Patterns::new().unwrap()));
        plugins.push(Box::new(RandomChat::new()));

        let log_dir = env::current_dir().unwrap().as_path().join("logs");

        BotCore {
            plugins: plugins,
            users: HashMap::new(),
            channels: HashMap::new(),
            logger: Logger::new(log_dir)
        }
    }

    pub fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str) {
        let user_name = if let Some(name) = self.users.get(user) { &name } else { user };
        let channel_name = if let Some(name) = self.channels.get(channel) { &name } else { channel };
        let _ = self.logger.log(format!("<{}> {}", user_name, msg));

        self.plugins.sort_by_key(|x| x.plugin_priority(user, channel, msg));
        for plugin in (&mut self.plugins).into_iter() {
            let result = 
                if msg.starts_with(&SETTINGS.command_char) {
                    let params = msg[SETTINGS.command_char.len()..].split_whitespace().map(|x| x.to_lowercase()).collect();
                    plugin.handle_command(user_name, channel_name, params)
                }
                else {
                    plugin.handle_message(user_name, channel_name, msg)
                };
            let resume = result.resume_mode();
            match result {
                BotEvent::Log(message, _) => {
                    let _ = self.logger.log(message);
                },
                BotEvent::Send(message, _) => {
                    if let Err(e) = client.send_message(channel, &message) {
                        let _ = self.logger.log(format!("***ERROR: Couldn't send message: {:?}", e));
                    }
                    else {
                        let _ = self.logger.log(format!("<{}> {}", client.get_name().unwrap(), &message));
                    }
                },
                BotEvent::None(_) => ()
            }
            if resume == ResumeEventHandling::Stop {
                break;
            }
        }
    }

}

impl EventHandler for BotCore {

    fn on_event(&mut self, client: &mut RtmClient, event: Result<&Event, Error>, _: &str) {
        if event.is_err() {
            return;
        }

        match *(event.unwrap()) {
            Event::Message(ref msg) => {
                match msg.clone() {
                    Message::Standard { user, text, channel, .. } => {
                        let user = user.unwrap();
                        let channel = channel.unwrap();
                        let text = text.unwrap();
                        self.handle_message(client, &user, &channel, &text);
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn on_ping(&mut self, _: &mut RtmClient) {
        let _ = self.logger.log_with_mode("Ping!", LogMode::Console);
    }

    fn on_close(&mut self, _: &mut RtmClient) {
        let _ = self.logger.log("*** Disconnected ***");
    }

    fn on_connect(&mut self, client: &mut RtmClient) {
        let users = client.get_users();
        for user in users.into_iter() {
            let prefix = if Some(true) == user.is_primary_owner { "&" }
                         else if Some(true) == user.is_owner { "~" }
                         else if Some(true) == user.is_admin { "@" }
                         else { "" };
            self.users.insert(user.id.clone(), format!("{}{}", prefix, &user.name));
        }

        let channels = client.get_channels();
        for channel in channels.into_iter() {
            self.channels.insert(channel.id.clone(), channel.name.clone());
        }
        
        let _ = self.logger.log("*** Connected to Slack ***");
    }
}

fn main() {
    let mut client = RtmClient::new(&SETTINGS.token);
    let mut handler = BotCore::new();

    if let Err(e) = client.login_and_run::<BotCore>(&mut handler) {
        println!("{:?}", e);
    }
}
