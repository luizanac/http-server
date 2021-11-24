use crate::{ParseError, Request, Response, Route, StatusCode};
use std::io::Read;
use std::net::TcpListener;
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server<'a> {
    addr: String,
    routes: Option<Vec<Route<'a>>>,
}

impl<'a> Server<'a> {
    pub fn new(addr: String) -> Self {
        Self { addr, routes: None }
    }

    pub fn run<T: Handler>(self, mut handler: T) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Data readed from connection: {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                print!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to create a connection: {}", e),
            }
        }
    }
}
