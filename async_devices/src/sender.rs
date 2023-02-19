use crate::thermo::Thermo;
use std::error::Error;
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

pub struct UdpSender {}

impl UdpSender {
    pub async fn start_send(
        send_addr: String,
        recv_addr: String,
        thermo: Thermo,
    ) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(send_addr).await?;

        tokio::spawn(async move {
            loop {
                let msg = format!("{} - {}\t", thermo.name, thermo.value);
                if let Err(e) = socket.send_to(msg.as_bytes(), recv_addr.as_str()).await {
                    println!("cannot send data to {}: witn err {}", recv_addr, e);
                }
                sleep(Duration::from_millis(1_000)).await;
            }
        });
        Ok(Self {})
    }
}
