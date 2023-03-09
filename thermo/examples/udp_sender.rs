use thermo::thermo::Thermo;
use thermo::udp_sender::*;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // может через std::env::args
    let thermo = Thermo::new("test".to_owned(), 21.2);
    let addr1 = "127.0.0.1:7878".to_owned();
    let addr2 = "127.0.0.1:3000".to_owned();

    UdpSender::start_send(addr1, addr2, thermo)?;
    loop {
        thread::sleep(Duration::new(1, 0));
    }
}
