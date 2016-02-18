//response_handler_tests.rs


#[cfg(test)]

mod get_content_type_tests{
    use response_handler::{get_content_type};

    #[test]
    fn test_for_html (){
        let file_check = "hello.html".to_string();
        let check = get_content_type(file_check.clone());
        assert_eq!((get_content_type(file_check.clone())), "text/html".to_string());
    }

    #[test]
    fn test_for_other(){
        let file_check = "hello.txt".to_string();
        assert_eq!((get_content_type(file_check.clone())), "text/plain".to_string());
    }
}

mod make_response_tests{
    use response_handler::{make_response};
    //use main::{Request};
    use Request;
    /*pub struct Request {
        method: String,
        file_path: String,
        protocol: String,
    }*/

    #[test]
    fn response_tester(){

    let status = "OK".to_string();
    let r = Request{
        method: "GET".to_string(),
        file_path: "hello.txt".to_string(),
        protocol: "HTTP".to_string(),
    };

    let test1 = make_response (&r, "status_code", "payload".to_string());
    assert_eq!(test1.protocol, "HTTP".to_string());
    assert_eq!(test1.method, "GET".to_string());
    assert_eq!(test1.status_code, "status_code".to_string());
    assert_eq!(test1.content_type, "text/plain".to_string());
    assert_eq!(test1.content_length, 7);
    assert_eq!(test1.payload, "payload".to_string());
}
}
