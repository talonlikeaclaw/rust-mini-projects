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
    let mut reader = io::BufReader::new(stream.try_clone().expect("stream clone"));
    // Send prompt
    stream.write_all(b"Enter username: ").expect("write");
    loop {
        let mut line_buf = String::new();
        match reader.read_line(&mut line_buf) {
            Ok(0) => {
                /* client closed before giving a nick */
                break;
            }
            Ok(_) => {
                let nick = line_buf.trim().to_string();

                // reject empty names or names already taken
                if nick.is_empty() {
                    stream.write_all(b"Name cannot be empty\n").unwrap();
                    continue;
                }

                // check uniqueness in the shared map
                let mut clist = clients.lock().unwrap();
                if clist.values().any(|c| c.username.as_deref() == Some(&nick)) {
                    stream
                        .write_all(b"Name already taken, try another\n")
                        .unwrap();
                    continue;
                }

                // we have a valid nick → store it and break the loop
                let client = Client {
                    stream: stream.try_clone().expect("stream clone"),
                    address: address.clone(),
                    username: Some(nick.clone()),
                };
                clist.insert(address.clone(), client);
                drop(clist); // unlock before continuing

                stream.write_all(b"Welcome!\n").unwrap();
                break;
            }
            Err(_) => {
                println!("Error reading nick from {}", address);
                return;
            }
        }
    }

    let mut buffer = [0u8; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnect
                println!("Client {} disconnected", address);
                break;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                // If the client sent nothing but a newline we ignore it
                if !msg.trim().is_empty() {
                    broadcast_message(&clients, &address, &msg);
                }
            }
            Err(_) => {
                println!("Error reading from {}", address);
                break;
            }
        }
    }

    println!("Client handler thread ending for {}", address);
}

fn broadcast_message(
    clients: &Arc<Mutex<HashMap<String, Client>>>,
    sender_address: &str,
    message: &str,
) {
    // Copy addresses to avoid lock issues
    let addresses: Vec<String> = {
        let client_list = clients.lock().unwrap();
        client_list.keys().cloned().collect()
    };

    let full_message = format!("{}: {}\n", sender_address, message.trim());

    for address in addresses {
        if address != sender_address {
            let mut client_list = clients.lock().unwrap(); // Lock for each

            if let Some(client) = client_list.get_mut(&address) {
                if client.stream.write(full_message.as_bytes()).is_err() {
                    // TODO: Handle write error
                }
            }
            // Lock is released here
        }
    }
    // And here
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
