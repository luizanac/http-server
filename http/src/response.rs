use std::io::{Result as IoResult, Write};

use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send<T: Write>(&self, stream: &mut T) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\rn\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            &body
        )
    }
}
