
use rand::prelude::*;
use std::io::Result;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let listen_addr = listener.local_addr()?;
    println!("Listenting on {}", listen_addr);
    
    loop {
        let (socket, remote_addr) = listener.accept().await?;
        let (rd, wr) = io::split(socket);
        
        tokio::spawn(async move {
            match handle_read(rd, &remote_addr).await {
                Ok(()) => println!("Remote {} read hung up gracefully", remote_addr),
                Err(e) => println!("Remote {} read hung up with error {}", remote_addr, e),
            };
        });

        tokio::spawn(async move {
            match handle_write(wr).await {
                Ok(()) => println!("Remote {} write hung up gracefully", remote_addr),
                Err(e) => println!("Remote {} write hung up with error {}", remote_addr, e),
            }
        });
    }
}

async fn handle_read(mut read_socket: ReadHalf<TcpStream>, remote_addr: &SocketAddr) -> Result<()> {
    let mut buf = vec![0; 4096];

    loop {
        let bytes_read = read_socket.read(&mut buf).await?;

        if bytes_read == 0 {
            return Ok(());
        }

        println!("Read {} bytes from {}", bytes_read, remote_addr);
    }
}

async fn handle_write(mut write_socket: WriteHalf<TcpStream>) -> Result<()> {
    let mut buf: Vec<u8> = vec![0; 4096];

    loop {
        thread_rng().fill(&mut buf[..]);
        write_socket.write_all(&buf).await?;
    }
}
