
extern crate handlers;

use chrono::prelude::*;
use std::io::Result;
use tokio::net::TcpListener;
use tokio::io;

#[tokio::main]
pub async fn main() -> Result<()> {
    let startup_time = Local::now();
    println!("Started at {}", startup_time.to_rfc3339());

    let listen_addr = std::env::args().nth(1).unwrap_or(String::from("127.0.0.1:0"));

    let listener = TcpListener::bind(&listen_addr).await?;
    let effective_listen_addr = listener.local_addr()?;
    println!("Listenting on {} ({})", &effective_listen_addr, &listen_addr);

    loop {
        let (socket, remote_addr) = listener.accept().await?;
        let (rd, wr) = io::split(socket);

        println!("Handling connection from {}", remote_addr);

        tokio::spawn(async move {
            let result = handlers::read(rd).await;
            let event_time = Local::now().to_rfc3339();
            match result {
                Ok(()) => println!("{}: Remote {} read hung up gracefully", event_time, remote_addr),
                Err(e) => println!("{}: Remote {} read hung up with error {}", event_time, remote_addr, e),
            };
        });

        tokio::spawn(async move {
            let result = handlers::write(wr).await;
            let event_time = Local::now().to_rfc3339();
            match result {
                Ok(()) => println!("{}: Remote {} write hung up gracefully", event_time, remote_addr),
                Err(e) => println!("{}: Remote {} write hung up with error {}", event_time, remote_addr, e),
            }
        });
    }
}
