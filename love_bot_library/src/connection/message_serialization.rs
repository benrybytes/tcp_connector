use std::{fs::File, io::{Cursor, Error, Write}, net::SocketAddr, process::{Command, Stdio}};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::{error, info};

use super::messages::{Message, MESSAGE_HEADER_LENGTH};

pub fn get_message_to_buffer_big_endian(type_and_length: [u8; MESSAGE_HEADER_LENGTH]) -> usize {
    let msg_length_off = &type_and_length[0..MESSAGE_HEADER_LENGTH];
    // in-memory buffer
    let mut rdr = Cursor::new(msg_length_off);
    rdr.read_u32::<BigEndian>().unwrap() as usize
}

pub fn save_screenshot(buffer: &Vec<u8>, height: u32, width: u32, peer_address: &SocketAddr) {
}

pub fn parse_message(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Find the part of the string that represents the byte array
    if let Some(start) = input.find('[') {
        if let Some(end) = input.find(']') {
            let byte_str = &input[start + 1..end]; // Extract the part between '[' and ']'

            // Split by commas, trim whitespace, and parse each number as a byte
            let bytes: Vec<u8> = byte_str
                .split(',')
                .map(|s| s.trim().parse::<u8>())
                .collect::<Result<_, _>>()?; // Collect into a Vec<u8>

            // Convert bytes to a string
            let result = String::from_utf8(bytes)?;
            return Ok(result);
        }
    }

    Err("Invalid input format".into())
}

pub fn turn_to_bytes(response: Message) -> Result<Vec<u8>, Error> {
    let turn_to_string = ron::ser::to_string(&response).unwrap_or_else(|e| {
        panic!("couldn't turn to string :c {}", e.to_string());
    });

    let message_len = turn_to_string.len();
    let mut buffer: Vec<u8> = Vec::with_capacity(message_len + MESSAGE_HEADER_LENGTH);
    buffer.write_u32::<BigEndian>(message_len as u32).expect("could not write size"); // write bytes to buffer
    buffer.extend(turn_to_string.into_bytes()); // fill with message
    Ok(buffer)
}
