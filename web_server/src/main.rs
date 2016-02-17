// main.rs
extern crate chrono;

use chrono::*;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::env;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;


// constants
const SERVER_NAME: &'static str = "nsg622-dlq200-web-server/0.1";

struct Request {
    method: String,
    file_path: String,
    protocol: String,
}

struct Response {
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
    let mut log_file_handle = File::create("log.txt").unwrap();
    let log_file = Arc::new(Mutex::new(log_file_handle));

    for stream in listener.incoming() {
        // spawn new thread for new connection
        let log_file = log_file.clone();

        thread::spawn(move|| {
            println!("New connection!");
            // grab TcpStream from incoming connections of TcpListener
            let mut stream = stream.unwrap();
            let input_line = read_from_stream(&mut stream);
            match parse_input(input_line) {
                Ok(request) => {
                    println!("Correct request received!");
                    let file_contents = read_from_file(request.file_path.clone());
                    let response: Response;
                    match file_contents {
                        Ok(file_text) => {
                            response = make_response(&request, "200", file_text);
                        },
                        Err(err_kind) => {
                            if err_kind == ErrorKind::NotFound {
                                response = make_response(&request, "404", "".to_string());
                            } else if err_kind == ErrorKind::PermissionDenied {
                                response = make_response(&request, "403", "".to_string());
                            } else {
                                panic!("Error occurred with reading from file!");
                            }
                        }
                    }
                    // write response to stream
                    log_to_file(&request, &response, &log_file);
                    print_response(&mut stream, response);
                },
                Err(err_kind) => {
                    if err_kind == ErrorKind::InvalidInput {
                        // write response straight to stream since no request exists
                        let response_text = "400 Bad Request \n".to_string();
                        stream.write(response_text.as_bytes());
                    }
                }
            }
        });
    }
}

fn read_from_stream(stream: &mut TcpStream) -> String {
    const BUF_SIZE: usize = 128;
    let mut buf = [0; BUF_SIZE];
    let mut result = String::new();
    let mut addition: String;

    // continually pass in a buffer until nothing left to read
    while let Ok(length) = stream.read(&mut buf[..]) {
        // add data in buffer to results string
        addition = String::from_utf8(buf.to_owned()).unwrap();
        result.push_str(&addition);
        buf = [0; BUF_SIZE];

        // break if all of input has been read
        if length < BUF_SIZE {
            break;
        }
    }

    result
}

fn parse_input(input: String) -> Result<Request, ErrorKind> {
    println!("{}", input);

    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 3 || tokens[0] != "GET" || !tokens[2].contains("HTTP")  {
        return Err(ErrorKind::InvalidInput);
    }

    // let protocol = tokens[2];
    // let protocol_tokens: Vec<&str> = protocol.split("\n").collect();

    let request = Request {
        method: tokens[0].to_string(),
        file_path: normalize_file_path(tokens[1].to_string()),
        // protocol: protocol_tokens[0].to_owned(),
        protocol: tokens[2].to_string(),
    };

    Ok(request)
}

fn normalize_file_path(file_path: String) -> String {
    let slash_index = file_path.find('/');
    match slash_index {
        Some(index) => {
            if index == 0 {
                let slice = &file_path[1..];
                return slice.to_owned();
            } else {
                return file_path;
            }
        },
        None => return file_path
    }
}

fn read_from_file(file_path: String) -> Result<String, ErrorKind> {
    let mut path = env::current_dir().unwrap();
    path.push(file_path);
    let file = File::open(path);
    match file {
        Ok(mut f) => {
            let mut buffer = String::new();
            let _ = f.read_to_string(&mut buffer);
            Ok(buffer)
        },
        Err(e) => {
            Err(e.kind())
        }
    }
}

fn get_content_type(file_path: String) -> String {
    let mut tokens: Vec<&str> = file_path.split(".").collect();
    let extension = tokens.pop().unwrap();
    if extension == "html" {
        "text/html".to_string()
    } else {
        "text/plain".to_string()
    }
}

fn make_response(request: &Request, status_code: &str, payload: String) -> Response {
    Response {
        protocol: request.protocol.clone(),
        method: request.method.clone(),
        status_code: status_code.to_string(),
        content_type: get_content_type(request.file_path.clone()),
        content_length: payload.len(),
        payload: payload,
    }
}

fn print_response(stream: &mut TcpStream, response: Response) {
    let mut response_text: String = "HTTP/1.0 ".to_string();
    // response_text = response_text + &response.protocol;
    // response_text = response_text + &" ";
    response_text = response_text + &response.status_code;
    // response_text = response_text + &" ";

    if &response.status_code == &"200" {
        response_text = response_text + &" OK\n";
        response_text = response_text + &SERVER_NAME + &"\n";
        response_text = response_text + &"Content-type: " + &response.content_type + &"\n";
        response_text = response_text + &"Content-length: " + &response.content_length.to_string() + &"\n";
        response_text = response_text + &"\n\n";
        response_text = response_text + &response.payload;
        response_text = response_text + &"\n\n";
    } else {
        if &response.status_code == &"404" {
            response_text = response_text + &" Not Found" + &"\n";
        } else if &response.status_code == &"400" {
            response_text = response_text + &" Bad Request" + &"\n";
        } else if &response.status_code == &"403" {
            response_text = response_text + &" Forbidden" + &"\n";
        }
    }

    stream.write(response_text.as_bytes());
}

fn log_to_file(request: &Request, response: &Response, log_file_mutex: &Arc<Mutex<File>>) {
    let mut file_guard = log_file_mutex.lock().unwrap();
    let mut current_date: DateTime<Local> = Local::now();
    let mut result = String::new();
    result = result + &"Request: " + &request.method + &" " + &request.file_path + &"\n";
    result = result + &"Response: " + &response.status_code + &" " + &response.content_type + &"\n";
    result = result + &"DateTime: " + &current_date.format("%Y-%m-%d %H:%M:%S").to_string() + &"\n";
    result = result + &"\n\n";
    file_guard.write(result.as_bytes());
}
