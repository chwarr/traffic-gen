
extern crate handlers;

use std::io::Result;
use tokio::net::TcpListener;
use tokio::io;

#[tokio::main]
pub async fn main() -> Result<()> {
    let listen_addr = std::env::args().nth(1).unwrap_or(String::from("127.0.0.1:0"));
    
    let listener = TcpListener::bind(&listen_addr).await?;
    let effective_listen_addr = listener.local_addr()?;
    println!("Listenting on {} ({})", &effective_listen_addr, &listen_addr);
    
    loop {
        let (socket, remote_addr) = listener.accept().await?;
        let (rd, wr) = io::split(socket);

        println!("Handling connection from {}", remote_addr);
        
        tokio::spawn(async move {
            match handlers::read(rd).await {
                Ok(()) => println!("Remote {} read hung up gracefully", remote_addr),
                Err(e) => println!("Remote {} read hung up with error {}", remote_addr, e),
            };
        });

        tokio::spawn(async move {
            match handlers::write(wr).await {
                Ok(()) => println!("Remote {} write hung up gracefully", remote_addr),
                Err(e) => println!("Remote {} write hung up with error {}", remote_addr, e),
            }
        });
    }
}
