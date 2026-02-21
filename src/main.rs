use std::env;

use std::io::{self,Write};

use anyhow::Result;

pub mod types;

pub mod network;

use crate::types::{RaftMessage,RequestVoteArgs};

use crate::network::server::start_server;

use crate::network::client::send_rpc;

#[tokio::main]
async fn main() -> Result<()>{

    let args: Vec<String> = env::args().collect();

    if(args.len()<2){
        println!("send a commend like this bro cargo run --(port)");
        return Ok(());
    }

    let my_port:u16 =args[1].parse()?;

    // strating the server 
    tokio::spawn(async move{
        if let Err(e)= start_server(my_port).await{
            eprintln!("the server crashed {:?}",e);
        }
    });
    // over here we are giving time for the server to bind that port 

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    loop{
        println!("enter the target port to send the message");
        io::stdout().flush()?;

        let mut input=String::new();
        io::stdin().read_line(&mut input);

        let trimmed=input.trim();

        if trimmed=="q"{
            println!("as u have entered we are quitting boom");
            break;
        }
        match trimmed.parse::<u16>(){
            Ok(target_port)=>{
                let target_addr=format!("127.0.0.1:{}",target_port);

                let vote_args = RequestVoteArgs{
                    term:1,
                    candidate_id:99,
                    last_log_index:0,
                    last_log_term:0,
                };

                let msg= RaftMessage::RequestVote(vote_args);

                if let Err(e)= send_rpc(&target_addr, msg).await{
                    println!("there is some error bro failed to send sad");
                }
            }
            Err(_)=>{
                println!("enter the correct port number nig");
            }
        }
    }
    Ok(())
}