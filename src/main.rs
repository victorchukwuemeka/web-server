use std::{
    fs,
    io::{prelude::*,  BufReader},
    net::{TcpListener, TcpStream},
};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
    .unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }

}


fn handle_connection(mut stream : TcpStream){
    let buf_read = BufReader::new(&mut stream );
    //let _http_request : Vec<_> = buf_read.lines()
    //.map(|result|result.unwrap())
    //.take_while(|line|!line.is_empty())
    //.collect();
    let request_line  = buf_read.lines().next().unwrap().unwrap();
     
    if request_line == "GET /HTTP/1.1" {
        let status_line ="HTTP/1.1 200";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length =contents.len();
        let response = format!(
            "{status_line}r\ncontent-length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }else {
        let status_line = "HTTp/1.1 404 NoT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}r\ncontent-length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}




