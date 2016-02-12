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
                        let file_contents = read_from_file(request.file_path);
                        match file_contents {
                            Ok(file_text) => {
                                println!("{}", file_text);
                            },
                            Err(err_kind) => {
                                if err_kind == ErrorKind::NotFound {
                                    println!("{}", "404 status".to_string());
                                } else if err_kind == ErrorKind::PermissionDenied {
                                    println!("{}", "403 status".to_string());
                                } else {
                                    panic!("Error occurred with reading from file!");
                                }
                            }
                        }
                    },
                    Err(err_kind) => {
                        if err_kind == ErrorKind::InvalidInput {
                            println!("404 status code");
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
    let mut file = File::open(path);
    match file {
        Ok(mut f) => {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer);
            Ok(buffer)
        },
        Err(e) => {
            Err(e.kind())
        }
    }
}
