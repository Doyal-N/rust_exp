use smart_socket::{server, socket::SmartSocket};

fn main() {
    let socket = SmartSocket::default();
    server::listen_client(socket).unwrap();
}
