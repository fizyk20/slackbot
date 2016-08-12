use slack::RtmClient;
use plugin::Plugin;

pub struct Logger;

impl Plugin for Logger {
    fn plugin_priority(&self, _: &str, _: &str, _: &str) -> i16 {
        -1
    }

    fn handle_message(&mut self, _: &mut RtmClient, user: &str, _: &str, msg: &str) -> bool {
        println!("<{}> {}", user, msg);
        false
    }
}
