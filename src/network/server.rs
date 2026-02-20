use tokio::net::TcpListener;
use anyhow::Result;
use crate::network::recieve_message;

pub async fn start_server(port: u16)-> Result<()>{

    let address = format!("127.0.0.1{}",port);
    let listener = TcpListener::bind(&address).await?;
    println!("the paxor node is listening on the address {}",address);
    loop{

        let (mut socket,peer_addr)=listener.accept().await?;
        println!("we have a new connection from {}",peer_addr);
        // tokio::spawn()
    }


    Ok(())
}