use super::{Request, Response, SERVER_NAME};

use std::net::TcpStream;
use std::io::Write;


pub fn make_response(request: &Request, status_code: &str, payload: String) -> Response {
    Response {
        protocol: request.protocol.clone(),
        method: request.method.clone(),
        status_code: status_code.to_string(),
        content_type: get_content_type(request.file_path.clone()),
        content_length: payload.len(),
        payload: payload,
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

pub fn print_response(stream: &mut TcpStream, response: Response) {
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

    stream.write(response_text.as_bytes()).expect("Returning HTTP response failed.");
}
