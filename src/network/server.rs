use tokio::net::TcpListener;
use tokio::sync::mpsc;

use anyhow::Result;
use crate::network::read_message;

use crate::types::RaftMessage;

pub async fn start_server(port: u16, tx:mpsc::Sender<RaftMessage>)-> Result<()>{

    let address = format!("127.0.0.1:{}",port);

    let listener = TcpListener::bind(&address).await?;

    println!("the paxor node is listening on the address {}",address);

    // okay so we have many nodes for each of the nodes we clone a new mailbox 
    
    loop{
        let (mut socket,peer_addr)=listener.accept().await?;
        println!("we have a new connection from {}",peer_addr);
        let tx_clone=tx.clone();
        tokio::spawn(async move{
            loop{
                match read_message(&mut socket).await{
                    Ok(msg)=>{
                        if tx_clone.send(msg).await.is_err(){
                            println!("the brain is dead bro so this message is dropped {:?}",peer_addr);
                        }
                        // println!("Recieved message {:?} from peer{}",msg,peer_addr)
                    }
                    Err(e)=>{
                        println!("connection closed by peer {} with error {:?}",peer_addr,e);
                        break;
                    }
                }
            }
    });
    }
}