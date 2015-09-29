extern crate loirc;

extern crate irc_to_http;

use std::thread;

use irc_to_http::irc_controller::*;
use irc_to_http::irc_controller::irc_configuration::*;

fn main() {
    let mut connection_config = IrcConfiguration::new();
    connection_config.host_name("euroserv.fr.quakenet.org".to_string()).port(6667).nick_name("gonzales".to_string()).user_name("gonzales".to_string());
    let mut controller = IrcController::new(connection_config);
    controller.start_handling();

    loop {
        thread::sleep_ms(100);
    }
}
