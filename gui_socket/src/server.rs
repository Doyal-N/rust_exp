use std::{
    io::{stdout, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::socket::SmartSocket;

pub const ADDR: &str = "127.0.0.1:7878";

pub fn listen_client(mut socket: SmartSocket) -> Result<(), String> {
    let listener = TcpListener::bind(ADDR).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_command(s, &mut socket),
            Err(e) => println!("socket communication with err - {}", e),
        }
    }

    Ok(())
}

fn handle_command(stream: TcpStream, socket: &mut SmartSocket) {
    let messages = BufReader::new(stream);

    for msg in messages.lines() {
        match msg.unwrap() {
            msg if msg.as_bytes() == b"switch" => {
                socket.switch();
                println!("socket switched");
            }
            msg if msg.as_bytes() == b"state" => {
                println!("state {} \n", socket.status());
                // stdout().write_all(soc.status().as_bytes()).unwrap();
            }
            _ => stdout().write_all(b"wrong command").unwrap(),
        }
    }
}
