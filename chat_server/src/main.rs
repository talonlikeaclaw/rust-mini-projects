use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    address: String,
}

fn handle_client(mut stream: TcpStream) {
    println!("Handling client connection!");

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnect
                println!("Client disconnected!");
                break;
            }
            Ok(n) => {
                // Convert bytes to string and print
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", message);

                // Echo the message back to client
                let response = format!("Echo: {}\n", message);
                if stream.write(response.as_bytes()).is_err() {
                    break; // Client disconnect
                }
            }
            Err(_) => {
                println!("Error reading from client");
                break;
            }
        }
    }

    println!("Client handler thread ending!");
}

fn main() -> std::io::Result<()> {
    // Bind to localhost port 5095
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5095);
    let listener = TcpListener::bind(socket_addr)?;
    println!("Chat server listening on {socket_addr}!");

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
