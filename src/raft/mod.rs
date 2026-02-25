use tokio::sync::mpsc;


use crate::types::{LogEntry,RaftMessage,RequestVoteArgs};

#[derive(Debug,PartialEq,Clone)]

pub enum NodeState{
    Follower,
    Candidate,
    Leader,
}

pub struct RaftNode{
    pub id:u64,
    
    pub state:NodeState,
    
    pub current_term:u64,

    pub voted_for:Option<u64>,

    pub log:Vec<LogEntry>,

    pub commit_index: u64,

    pub last_applied: u64,

    pub mailbox:mpsc::Receiver<RaftMessage>,
}

impl RaftNode{
    pub fn new(id:u64,mailbox:mpsc::Receiver<RaftMessage>)->Self{
        Self{
            id,
            state:NodeState::Follower,
            current_term:0,
            voted_for:None,
            log: Vec::new(),
            commit_index:0,
            last_applied:0,
            mailbox,
        }
    }
    pub async fn run(&mut self){
        println!("raft brain with node id{} and it has booted up as {:?}",self.id,self.state);
        while let Some(msg)= self.mailbox.recv().await{
            // println!("the brain recieved meessage {:?}",msg);
            match msg{
                RaftMessage::RequestVote(args)=>{
                    println!(" the mail box has recieved ur votes bro routing to loigc");
                    self.handle_request_vote(args).await;
                }
                _ =>{
                    println!("We have recieved an unhandles message {:?}",msg);
                }
            }
        }
    }
    async fn handle_request_vote(&mut self,args: RequestVoteArgs){
        let mut granted =false;
        
        if args.term < self.current_term{
            granted=false;
        }
        else if self.voted_for.is_none() || self.voted_for == Some(args.candidate_id){
            granted = true;
            self.current_term = args.term;
            self.voted_for = Some(args.candidate_id);
        }
        if granted{
            println!("the vote has been granted to node {} term {}",args.candidate_id,args.term);
        }
        else{
            println!("the vote has not been granted bruh its been denied node {} term {}",args.candidate_id,args.term);
        }
    }
}