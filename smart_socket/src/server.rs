use std::{
    io::{stdout, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::socket::SmartSocket;

pub const ADDR: &str = "127.0.0.1:7878";

pub fn listen_client() -> Result<(), String> {
    let listener = TcpListener::bind(ADDR).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_command(s),
            Err(e) => println!("socket communication with err - {}", e),
        }
    }

    Ok(())
}

fn handle_command(stream: TcpStream) {
    let messages = BufReader::new(stream);
    let mut soc = SmartSocket::new();

    for msg in messages.lines() {
        match msg.unwrap() {
            msg if msg.as_bytes() == "switch".as_bytes() => soc.switch(),
            msg if msg.as_bytes() == "state".as_bytes() => {
                let state = soc.status();
                stdout().write_all(state.as_bytes()).unwrap();
            }
            _ => println!("mismatched command"),
        }
    }
}
