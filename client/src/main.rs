
extern crate handlers;

use chrono::prelude::*;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::io;

#[tokio::main]
pub async fn main() -> Result<()> {
    let startup_time = Local::now();
    println!("Started at {}", startup_time.to_rfc3339());

    let remote_addr = std::env::args().nth(1).expect("no remote endpoint given");

    let tcp_stream = TcpStream::connect(&remote_addr).await?;
    let local_addr = tcp_stream.local_addr()?;
    let peer_addr = tcp_stream.peer_addr()?;
    println!("Connected to {} ({}) from {}", peer_addr, &remote_addr, local_addr);

    let (rd, wr) = io::split(tcp_stream);

    let read_handler = tokio::spawn(async move {
        let result = handlers::read(rd).await;
        let event_time = Local::now();
        match result {
            Ok(()) => println!("{}: Remote {} read hung up gracefully", event_time, peer_addr),
            Err(e) => println!("{}: Remote {} read hung up with error {}", event_time, peer_addr, e),
        };
    });

    let write_handler = tokio::spawn(async move {
        let result = handlers::write(wr).await;
        let event_time = Local::now();
        match result {
            Ok(()) => println!("{}: Remote {} write hung up gracefully", event_time, peer_addr),
            Err(e) => println!("{}: Remote {} write hung up with error {}", event_time, peer_addr, e),
        }
    });

    tokio::try_join!(read_handler, write_handler)?;
    Ok(())
}
