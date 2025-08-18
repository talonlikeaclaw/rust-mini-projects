use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("Handling client connection!");
}

fn main() -> std::io::Result<()> {
    // Bind to localhost port 5095
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5095);
    let listener = TcpListener::bind(socket_addr)?;
    println!("Chat server listening on {socket_addr}");

    // Accept connections in a loop
    for stream in listener.incoming() {
        let stream = stream?;
        println!("New client connected!");

        // Handle each client in a separate thread
        thread::spawn(move || {
            handle_client(stream);
        });
    }

    Ok(())
}
