// main.rs
use std::io::Read;
use std::net::TcpListener;
use std::thread;

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
                let line = String::from_utf8(buf.to_owned()).unwrap();
                println!("{}", line);
            }
        });
    }
}
