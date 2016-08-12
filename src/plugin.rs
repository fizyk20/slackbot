use slack::RtmClient;
use std::cmp::Ordering;

pub trait Plugin {
    fn handle_mode(&self, user: &str, channel: &str, msg: &str) -> i16;
    fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str) -> bool;
}
