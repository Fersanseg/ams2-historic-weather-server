mod requests;
mod types;

use std::{
  io::prelude::*, net::{TcpListener, TcpStream}
};

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
  let res = requests::get_date_param(&stream).unwrap();
  stream.write_all(res.as_bytes()).unwrap();
}
