#[macro_use]
extern crate lazy_static;
extern crate slack;
extern crate regex;

mod settings;

use slack::{RtmClient, EventHandler, Event, Error, Message};
use settings::SETTINGS;

struct TestHandler;

impl TestHandler {

    pub fn echo(&mut self, client: &mut RtmClient, msg: Message) {
        match msg {
            Message::Standard { user, text, channel, .. } => {
                let user = user.unwrap();
                let channel = channel.unwrap();
                let text = text.unwrap();
                println!("{}: {}", &user, &text);
                client.send_message(&channel, &text);
            },
            _ => ()
        }
    }

}

impl EventHandler for TestHandler {

    fn on_event(&mut self, client: &mut RtmClient, event: Result<&Event, Error>, raw_json: &str) {
        if event.is_err() {
            return;
        }

        match *(event.unwrap()) {
            Event::Message(ref msg) => {
                self.echo(client, msg.clone());
            },
            _ => ()
        }
    }

    fn on_ping(&mut self, client: &mut RtmClient) {
        println!("Ping!");
    }

    fn on_close(&mut self, client: &mut RtmClient) {
    }

    fn on_connect(&mut self, client: &mut RtmClient) {
    }
}

fn main() {
    let mut client = RtmClient::new(&SETTINGS.token);
    let mut handler = TestHandler;

    println!("Starting...");

    client.login_and_run::<TestHandler>(&mut handler).unwrap();

    println!("Finished.");
}
