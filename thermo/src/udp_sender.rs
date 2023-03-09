use crate::thermo::Thermo;
use std::error::Error;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// #[derive(Debug)]
pub struct UdpSender {}

impl UdpSender {
    pub fn start_send(
        send_addr: String,
        recv_addr: String,
        thermo: Thermo,
    ) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(send_addr)?;

        thread::spawn(move || loop {
            let msg = format!("{} - {}\t", thermo.name, thermo.value);
            if let Err(e) = socket.send_to(msg.as_bytes(), recv_addr.as_str()) {
                println!("cannot send data to {}: witn err {}", recv_addr, e);
            }
            thread::sleep(Duration::from_millis(1_000));
        });
        Ok(Self {})
    }
}
