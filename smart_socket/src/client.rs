use std::{io::Write, net::TcpStream};

pub const ADDR: &str = "127.0.0.1:7878";

pub fn connect() -> Result<TcpStream, String> {
    let client = TcpStream::connect(ADDR).map_err(|_| "bad socket connection")?;

    Ok(client)
}

pub fn check_state() {
    if let Ok(mut client) = connect() {
        client.write_all(b"state").unwrap();
    }
}

pub fn switch_socket() {
    if let Ok(mut client) = connect() {
        client.write_all(b"switch").unwrap();
    }
}
