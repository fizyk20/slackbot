use slack::RtmClient;
use plugin::Plugin;

pub struct EchoPlugin;

impl Plugin for EchoPlugin {
    fn handle_message(&mut self, client: &mut RtmClient, user: &str, channel: &str, message: &str) {
        client.send_message(channel, message);
    }
}
