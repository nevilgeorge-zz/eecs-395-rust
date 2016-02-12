// main.rs
use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::env;
use std::fs::File;
use std::thread;

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

    for stream in listener.incoming() {
        // spawn new thread for new connection
        thread::spawn(move|| {
            println!("New connection!");
            // grab TcpStream from incoming connections of TcpListener
            let mut stream = stream.unwrap();
            let mut buf = [0; 128];
            // pass in a buffer and read from connection
            let reading = stream.read(&mut buf[..]);
            if let Ok(_) = reading {
                // convert utf8 buffer to string
                let input_line = String::from_utf8(buf.to_owned()).unwrap();
                let result = parse_input(input_line);
                match result {
                    Ok(request) => {
                        println!("Correct request received!");
                        let file_contents = read_from_file(request.file_path.clone());
                        let response: Response;
                        match file_contents {
                            Ok(file_text) => {
                                response = make_response(request, "200", file_text);
                            },
                            Err(err_kind) => {
                                if err_kind == ErrorKind::NotFound {
                                    response = make_response(request, "404", "".to_string());
                                } else if err_kind == ErrorKind::PermissionDenied {
                                    response = make_response(request, "403", "".to_string());
                                } else {
                                    panic!("Error occurred with reading from file!");
                                }
                            }
                        }
                        println!("{}", response.payload);
                        println!("{}", response.status_code);
                        println!("{}", response.content_type);
                        println!("{}", response.content_length);
                    },
                    Err(err_kind) => {
                        if err_kind == ErrorKind::InvalidInput {
                            println!("400 status code");
                        }
                    }
                }
            }
        });
    }
}

fn parse_input(input: String) -> Result<Request, ErrorKind> {
    let tokens: Vec<&str> = input.split(" ").collect();
    if tokens.len() != 3 || tokens[0] != "GET" || !tokens[2].contains("HTTP")  {
        return Err(ErrorKind::InvalidInput);
    }

    let request = Request {
        method: tokens[0].to_string(),
        file_path: normalize_file_path(tokens[1].to_string()),
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

fn make_response(request: Request, status_code: &str, payload: String) -> Response {
    Response {
        protocol: request.protocol.clone(),
        method: request.method.clone(),
        status_code: status_code.to_string(),
        content_type: get_content_type(request.file_path),
        content_length: payload.len(),
        payload: payload,
    }
}
