#[macro_use]
extern crate lazy_static;
extern crate slack;
extern crate regex;
extern crate timer;
extern crate chrono;

mod settings;
mod plugin;
mod plugins;
mod logger;

use std::collections::HashMap;
use std::env;
use slack::{RtmClient, EventHandler, Event, Error, Message};
use settings::SETTINGS;
use logger::Logger;
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

        let log_dir = env::current_dir().unwrap().as_path().join("logs");

        BotCore {
            plugins: plugins,
            users: HashMap::new(),
            channels: HashMap::new(),
            logger: Logger::new(log_dir)
        }
    }

    pub fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str) {
        let _ = self.logger.log(format!("<{}> {}", user, msg));

        self.plugins.sort_by_key(|x| x.plugin_priority(user, channel, msg));
        for plugin in (&mut self.plugins).into_iter() {
            let result = plugin.handle_message(user, channel, msg);
            let resume = result.resume_mode();
            match result {
                BotEvent::Log(message, _) => {
                    let _ = self.logger.log(message);
                },
                BotEvent::Send(message, _) => {
                    if let Some(channel) = client.get_channel_id(channel) {
                        if let Err(e) = client.send_message(channel, &message) {
                            let _ = self.logger.log(format!("***ERROR: Couldn't send message: {:?}", e));
                        }
                        else {
                            let _ = self.logger.log(format!("<{}> {}", client.get_name().unwrap(), &message));
                        }
                    }
                    else {
                        let _ = self.logger.log(format!("***ERROR: No channel named {}", channel));
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
                        let user = if let Some(name) = self.users.get(&user) { name.clone() } else { user };
                        let channel = channel.unwrap();
                        let channel = if let Some(name) = self.channels.get(&channel) { name.clone() } else { channel };
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
        println!("Ping!");
    }

    fn on_close(&mut self, _: &mut RtmClient) {
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
    }
}

fn main() {
    let mut client = RtmClient::new(&SETTINGS.token);
    let mut handler = BotCore::new();

    println!("Starting...");

    if let Err(e) = client.login_and_run::<BotCore>(&mut handler) {
        println!("{:?}", e);
    }
    else {
        println!("Finished.");
    }
}
