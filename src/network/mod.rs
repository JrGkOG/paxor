pub mod server;
pub mod client;

use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::Result;

use crate::types::RaftMessage;

pub async fn send_message(stream: &mut TcpStream,msg:&RaftMessage)-> Result<()>{
    let serialized_data=wincode::serialize(msg)?;

    let len = serialized_data.len() as u64;

    stream.write_u64(len).await?;

    stream.write_all(&serialized_data).await?;

    Ok(())
}

pub async fn read_message(stream: &mut TcpStream)-> Result<RaftMessage>{
    let len=stream.read_u64().await?;

    let mut buffer = vec![0;len as usize];
    
    stream.read_exact(&mut buffer).await?;

    let msg: RaftMessage = wincode::deserialize(&buffer)?;

    Ok(msg)
}