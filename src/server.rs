use crate::http::Request;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            //TODO: Change to multi thread
            match listener.accept() {
                Ok((mut stream, _)) => {
                    //TODO: Read bodies with size more than 1024 bytes
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!(
                                "Data readed from connection: {}",
                                String::from_utf8_lossy(&buffer)
                            );

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    println!("")
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to estabilish a connection: {}", e),
            };
        }
    }
}
