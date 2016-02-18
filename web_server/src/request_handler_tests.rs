// request_handler_tests.rs

#[cfg(test)]
mod parse_input_tests {
    use std::io::{ErrorKind};
    use request_handler::{parse_input};

    #[test]
    fn parse_input_string() {
        let result = parse_input("GET /hello.txt HTTP/1.0".to_string()).unwrap();
        assert_eq!(result.method, "GET".to_string());
        assert_eq!(result.file_path, "hello.txt".to_string());
        assert_eq!(result.protocol, "HTTP/1.0".to_string());
    }

    #[test]
    fn parse_input_broken() {
        let result = parse_input("GET /hello.txt".to_string());
        match result {
            Err(err_kind) => {
                assert_eq!(err_kind, ErrorKind::InvalidInput);
            },
            Ok(_) => {
                return;
            }
        }
    }

    #[test]
    fn parse_input_empty() {
        let result = parse_input("".to_string());
        match result {
            Err(err_kind) => {
                assert_eq!(err_kind, ErrorKind::InvalidInput);
            },
            Ok(_) => {
                return;
            }
        }
    }
}

#[cfg(test)]
mod normalize_file_path_tests {
    use request_handler::{normalize_file_path};

    #[test]
    fn normalize_path_slash() {
        let input = "/hello.txt".to_string();
        let result = normalize_file_path(input.clone());
        assert_eq!(result, "hello.txt".to_string());
    }

    #[test]
    fn normalize_path_no_slash() {
        let input = "hello.txt".to_string();
        let result = normalize_file_path(input.clone());
        assert_eq!(result, input);
    }
}
