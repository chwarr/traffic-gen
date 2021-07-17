
use rand::prelude::*;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

#[tokio::main]
pub async fn main() -> Result<()> {
    let remote_addr = std::env::args().nth(1).expect("no remote endpoint given");

    let tcp_stream = TcpStream::connect(&remote_addr).await?;
    let peer_addr = tcp_stream.peer_addr()?;
    println!("Connected to {} ({})", peer_addr, &remote_addr);
    
    let (rd, wr) = io::split(tcp_stream);
        
    let read_handler = tokio::spawn(async move {
        match handle_read(rd).await {
            Ok(()) => println!("Remote {} read hung up gracefully", peer_addr),
            Err(e) => println!("Remote {} read hung up with error {}", peer_addr, e),
        };
    });

    let write_handler = tokio::spawn(async move {
        match handle_write(wr).await {
            Ok(()) => println!("Remote {} write hung up gracefully", peer_addr),
            Err(e) => println!("Remote {} write hung up with error {}", peer_addr, e),
        }
    });

    tokio::try_join!(read_handler, write_handler)?;
    Ok(())
}

async fn handle_read(mut read_socket: ReadHalf<TcpStream>) -> Result<()> {
    let mut buf = vec![0; 4096];

    loop {
        let bytes_read = read_socket.read(&mut buf).await?;

        if bytes_read == 0 {
            return Ok(());
        }
    }
}

async fn handle_write(mut write_socket: WriteHalf<TcpStream>) -> Result<()> {
    let mut buf: Vec<u8> = vec![0; 4096];

    loop {
        thread_rng().fill(&mut buf[..]);
        write_socket.write_all(&buf).await?;
    }
}
