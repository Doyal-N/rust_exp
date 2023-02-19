use crate::socket::SmartSocket;
use std::io::{self, stdout, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub const ADDR: &str = "127.0.0.1:7878";

pub async fn listen_client() -> io::Result<()> {
    let listener = TcpListener::bind(ADDR).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        handle_command(stream).await;
    }
}

async fn handle_command(stream: TcpStream) {
    let messages = BufReader::new(stream);
    let mut soc = SmartSocket::new();

    let mut lines = messages.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        match line {
            msg if msg.as_bytes() == "switch".as_bytes() => soc.switch().await,
            msg if msg.as_bytes() == "state".as_bytes() => {
                let state = soc.status();
                stdout().write_all(state.as_bytes()).unwrap();
            }
            _ => println!("mismatched command"),
        }
    }
}
