use super::server::Handler;
use crate::{Method, Request, Response, StatusCode};
use std::fs;
pub struct HttpHandler {
    public_path: String,
}

impl HttpHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(&path) {
            Ok(path) => {
                if path.starts_with(fs::canonicalize(&self.public_path).unwrap()) {
                    fs::read_to_string(path).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for HttpHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
