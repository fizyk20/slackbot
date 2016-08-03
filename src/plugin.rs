use slack::RtmClient;

pub trait Plugin {
    fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, msg: &str);
}
