//! main.rs

use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our server

    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind address.");
    let local_addr = listener.local_addr().expect("Could not get local address.");
    println!("Server running at {}", local_addr);

    run(listener)?.await
}
