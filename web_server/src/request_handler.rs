use super::{Request};

use std::net::TcpStream;
use std::fs::File;
use std::env;
use std::io::{ErrorKind, Read};

pub fn read_from_stream(stream: &mut TcpStream) -> String {
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

pub fn parse_input(input: String) -> Result<Request, ErrorKind> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 3 || tokens[0] != "GET" || !tokens[2].contains("HTTP")  {
        return Err(ErrorKind::InvalidInput);
    }

    let request = Request {
        method: tokens[0].to_string(),
        file_path: normalize_file_path(tokens[1].to_string()),
        protocol: tokens[2].to_string(),
    };

    Ok(request)
}

pub fn normalize_file_path(file_path: String) -> String {
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

pub fn read_from_file(file_path: String) -> Result<String, ErrorKind> {
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
