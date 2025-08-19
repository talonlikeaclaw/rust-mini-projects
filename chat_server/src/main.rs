use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    address: String,
    username: Option<String>,
}

fn handle_client(
    clients: Arc<Mutex<HashMap<String, Client>>>,
    mut stream: TcpStream,
    address: String,
) {
    println!("Client {} connected", address);

    let client = Client {
        stream: stream.try_clone().expect("Failed to clone stream"),
        address: address.clone(),
        username: None, // TODO: implement username feature
    };

    // Add client to shared list
    {
        let mut client_list = clients.lock().unwrap();
        client_list.insert(address.clone(), client);
    }

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnect
                println!("Client {} disconnected", address);
                break;
            }
            Ok(n) => {
                // Convert bytes to string and print
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}: {}", address, message.trim());

                // Echo the message back to client
                let response = format!("Echo: {}\n", message);
                if stream.write(response.as_bytes()).is_err() {
                    break; // Client disconnect
                }
            }
            Err(_) => {
                println!("Error reading from client {}", address);
                break;
            }
        }
    }

    println!("Client handler thread ending for {}", address);
}

fn main() -> std::io::Result<()> {
    // Bind to localhost port 5095
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5095);
    let listener = TcpListener::bind(socket_addr)?;
    println!("Chat server listening on {socket_addr}");

    // Shared list of clients
    let clients = Arc::new(Mutex::new(HashMap::new()));

    // Accept connections in a loop
    for stream in listener.incoming() {
        let stream = stream?;
        let address = stream.peer_addr().unwrap().to_string();
        let clients_clone = Arc::clone(&clients);
        println!("New client connecting from {}", address);

        // Handle each client in a separate thread
        thread::spawn(move || {
            handle_client(clients_clone, stream, address);
        });
    }

    Ok(())
}
