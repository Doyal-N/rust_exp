use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub const ADDR: &str = "127.0.0.1:7878";

pub async fn connect() -> Result<TcpStream, String> {
    let client = TcpStream::connect(ADDR)
        .await
        .map_err(|_| "bad socket connection")?;

    Ok(client)
}

pub async fn check_state() {
    if let Ok(mut client) = connect().await {
        let _ = client.write_all(b"state").await;
    }
}

pub async fn switch_socket() {
    if let Ok(mut client) = connect().await {
        let _ = client.write_all(b"switch").await;
    }
}
