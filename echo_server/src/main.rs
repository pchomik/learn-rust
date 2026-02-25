// Based on https://www.youtube.com/watch?v=EFxSVqbuAUE

use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    // Read IP address of new client
    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "unknown".to_string(), |addr| addr.to_string());
    println!("Handling connection from {}", peer_addr);

    // Read data from client and send it back
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Client {} close connection", peer_addr);
                    break;
                }
                if let Err(e) = stream.write_all(&buffer[0..n]) {
                    eprintln!(
                        "Was not able to write data to client {} because of error {}",
                        peer_addr, e
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "Was not able to read data from client {} because of error {}",
                    peer_addr, e
                );
                break;
            }
        }
    }
}

fn main() {
    // Read custom IP:PORT values from program arguments. Use default if not provided.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9090".to_string());

    // Bind application to IP:PORT
    let listener = TcpListener::bind(&addr).expect("Failed to bind server");

    println!("Server listening on {}", addr);

    // Accept new connections. For each new thread will be spawned.
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Failed to accept connection {}", e),
        }
    }
}
