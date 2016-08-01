extern crate slack;

use slack::{RtmClient, EventHandler, Event, Error, Message};

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
    let mut client = RtmClient::new("token-here");
    let mut handler = TestHandler;

    client.login_and_run::<TestHandler>(&mut handler);

    println!("Hello, world!");
}
