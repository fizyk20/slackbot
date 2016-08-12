use slack::RtmClient;

pub trait Plugin {
    fn plugin_priority(&self, user: &str, channel: &str, msg: &str) -> i16;
    fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str) -> bool;
}
