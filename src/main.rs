use std::{
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879")
    .unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }

}


fn handle_connection(mut stream : TcpStream){
    let buf_read = BufReader::new(&mut stream );
    let http_request : Vec<_> = buf_read.lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty())
    .collect();

    println!("Request : {http_request:#?}");
}