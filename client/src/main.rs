
extern crate handlers;

use std::io::Result;
use tokio::net::TcpStream;
use tokio::io;

#[tokio::main]
pub async fn main() -> Result<()> {
    let remote_addr = std::env::args().nth(1).expect("no remote endpoint given");

    let tcp_stream = TcpStream::connect(&remote_addr).await?;
    let local_addr = tcp_stream.local_addr()?;
    let peer_addr = tcp_stream.peer_addr()?;
    println!("Connected to {} ({}) from {}", peer_addr, &remote_addr, local_addr);

    let (rd, wr) = io::split(tcp_stream);

    let read_handler = tokio::spawn(async move {
        match handlers::read(rd).await {
            Ok(()) => println!("Remote {} read hung up gracefully", peer_addr),
            Err(e) => println!("Remote {} read hung up with error {}", peer_addr, e),
        };
    });

    let write_handler = tokio::spawn(async move {
        match handlers::write(wr).await {
            Ok(()) => println!("Remote {} write hung up gracefully", peer_addr),
            Err(e) => println!("Remote {} write hung up with error {}", peer_addr, e),
        }
    });

    tokio::try_join!(read_handler, write_handler)?;
    Ok(())
}
