mod requests;
mod types;

use std::{
  io::prelude::*, net::{TcpListener, TcpStream}
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
  let date = requests::get_date_param(&stream).unwrap();
  let status_line = "HTTP/1.1 200 OK";
  let date_length = date.len();

  let response = format!("{status_line}{CRLF}Content-Length: {date_length}{CRLF}{CRLF}{date}");

  stream.write_all(response.as_bytes()).unwrap();
}
