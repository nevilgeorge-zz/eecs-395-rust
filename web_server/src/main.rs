// main.rs
use std::io::Read;
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
                        println!("{}", file_contents);
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        });
    }
}

fn parse_input(input: String) -> Result<Request, String> {
    let tokens: Vec<&str> = input.split(" ").collect();
    if tokens.len() != 3 || tokens[0] != "GET" || !tokens[2].contains("HTTP")  {
        return Err("400 Bad Request".to_owned());
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

fn read_from_file(file_path: String) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(file_path);
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    buffer

}
