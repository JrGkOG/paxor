use tokio::net::TcpStream;

use anyhow::Result;

use crate::types::RaftMessage;

use crate::network::send_message;

pub async fn send_rpc(target_address:&str,message: RaftMessage)->Result<()>{
    
    let mut stream = TcpStream::connect(target_address).await?;
    println!("connected to target {}",target_address);
    println!("sending message now");
    send_message(&mut stream, &message).await?;
    println!("message sucessfully sent to target address");
    Ok(())
}
