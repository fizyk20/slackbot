#[macro_use]
extern crate lazy_static;
extern crate slack;
extern crate regex;

mod settings;
mod plugin;
mod plugins;

use std::collections::HashMap;
use slack::{RtmClient, EventHandler, Event, Error, Message};
use settings::SETTINGS;
use plugin::Plugin;
use plugins::*;

struct BotHandler {
    plugins: Vec<Box<Plugin>>,
    users: HashMap<String, String>,
    channels: HashMap<String, String>
}

impl BotHandler {

    fn new() -> BotHandler {
        let mut plugins : Vec<Box<Plugin>> = Vec::new();

        // load all used plugins
        plugins.push(Box::new(Logger));

        BotHandler {
            plugins: plugins,
            users: HashMap::new(),
            channels: HashMap::new()
        }
    }

    pub fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str) {
        self.plugins.sort_by_key(|x| x.plugin_priority(user, channel, msg));
        for plugin in (&mut self.plugins).into_iter() {
            if plugin.handle_message(client, user, channel, msg) {
                break;
            }
        }
    }

}

impl EventHandler for BotHandler {

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
        println!("Ping!");
    }

    fn on_close(&mut self, _: &mut RtmClient) {
    }

    fn on_connect(&mut self, _: &mut RtmClient) {
    }
}

fn main() {
    let mut client = RtmClient::new(&SETTINGS.token);
    let mut handler = BotHandler::new();

    println!("Starting...");

    client.login_and_run::<BotHandler>(&mut handler).unwrap();

    println!("Finished.");
}
