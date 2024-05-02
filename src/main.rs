use std::{
  fmt::format, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

const CRLF: &str = "\r\n";

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  println!("Server running on port 7878");
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  println!("Handling connection");
  let buf_reader = BufReader::new(&mut stream);
  let http_req: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let status = format!("HTTP/1.1 200 OK{CRLF}{CRLF}");
  let body = "Hello!";

  let response = format!("{status}{CRLF}{CRLF}{body}");

  stream.write_all(response.as_bytes()).unwrap();
}
