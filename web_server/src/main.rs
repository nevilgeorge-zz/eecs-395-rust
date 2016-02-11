// main.rs
use std::io::Read;
use std::net::TcpListener;
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
        thread::spawn(|| {
            println!("New connection!");
            // grab TcpStream
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
        file_path: tokens[1].to_string(),
        protocol: tokens[2].to_string(),
    };

    Ok(request)
}
