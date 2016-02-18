// main.rs
#[doc="
web_server

Nevil George nsg622
Diane Liu dlq200
Thornton Uhl tcu406

Basic web server that handles HTTP/0.9 GET requests.

Usage: cargo run
Starts the web server.
Listens on localhost:8080

ASSUMPTIONS:
*
"]

extern crate chrono;

use chrono::*;
use std::io::{ErrorKind, Write};
use std::net::TcpListener;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

mod request_handler;
mod response_handler;
mod request_handler_tests;
mod response_handler_tests;

const SERVER_NAME: &'static str = "nsg622-dlq200-tcu406-web-server/0.1";

pub struct Request {
    method: String,
    file_path: String,
    protocol: String,
}

pub struct Response {
    protocol: String,
    method: String,
    status_code: String,
    content_type: String,
    content_length: usize,
    payload: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Ready to accept connections on port 8080.");

    // create log file
    let log_file_handle = File::create("log.txt").unwrap();
    let log_file = Arc::new(Mutex::new(log_file_handle));

    for stream in listener.incoming() {
        // spawn new thread for new connection
        let log_file = log_file.clone();

        thread::spawn(move|| {
            println!("New connection!");
            // grab TcpStream from incoming connections of TcpListener
            let mut stream = stream.unwrap();
            let input_line = request_handler::read_from_stream(&mut stream);
            match request_handler::parse_input(input_line) {
                Ok(request) => {
                    println!("Correct request received!");
                    let file_contents = request_handler::read_from_file(request.file_path.clone());
                    let response: Response;
                    match file_contents {
                        Ok(file_text) => {
                            response = response_handler::make_response(&request, "200", file_text);
                        },
                        Err(err_kind) => {
                            if err_kind == ErrorKind::NotFound {
                                response = response_handler::make_response(&request, "404", "".to_string());
                            } else if err_kind == ErrorKind::PermissionDenied {
                                response = response_handler::make_response(&request, "403", "".to_string());
                            } else {
                                panic!("Error occurred with reading from file!");
                            }
                        }
                    }
                    // write response to stream
                    log_to_file(&request, &response, &log_file);
                    response_handler::print_response(&mut stream, response);
                },
                Err(err_kind) => {
                    if err_kind == ErrorKind::InvalidInput {
                        // write response straight to stream since no request exists
                        let response_text = "400 Bad Request \n".to_string();
                        stream.write(response_text.as_bytes()).expect("Returning 400 Bad Request failed.");
                    }
                }
            }
        });
    }
}


fn log_to_file(request: &Request, response: &Response, log_file_mutex: &Arc<Mutex<File>>) {
    let mut file_guard = log_file_mutex.lock().unwrap();
    let current_date: DateTime<Local> = Local::now();
    let mut result = String::new();
    result = result + &"Request: " + &request.method + &" " + &request.file_path + &"\n";
    result = result + &"Response: " + &response.status_code + &" " + &response.content_type + &"\n";
    result = result + &"DateTime: " + &current_date.format("%Y-%m-%d %H:%M:%S").to_string() + &"\n";
    result = result + &"\n\n";
    file_guard.write(result.as_bytes()).expect("Writing to log failed.");
}
