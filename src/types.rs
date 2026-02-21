use wincode::{SchemaRead,SchemaWrite};
// use std::fmt;

#[derive(Debug,Clone,SchemaWrite,SchemaRead)]

pub struct LogEntry{
    pub term: u64,
    pub index: u64, 
    pub command: Vec<u8>,
}

#[derive(Debug,Clone,SchemaRead,SchemaWrite)]
pub struct RequestVoteReply{
    pub term: u64,
    pub vote_granted: bool,
}

#[derive(Debug,Clone,SchemaRead,SchemaWrite)]
pub struct RequestVoteArgs{
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug,Clone,SchemaRead,SchemaWrite)]
pub struct AppendEntriesArgs{
    pub term :u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: Vec<LogEntry>,
    pub leader_commit: u64,
}

#[derive(Debug,Clone,SchemaRead,SchemaWrite)]
pub struct AppendEntriesReply{
    pub term: u64,
    pub success: bool,
    pub conflict_index : u64,
    pub conflict_term: u64,
}


#[derive(Debug,Clone,SchemaRead,SchemaWrite)]
pub enum RaftMessage{
    RequestVote(RequestVoteArgs),
    RequestVoteResponse(RequestVoteReply),
    AppendEntries(AppendEntriesArgs),
    AppendEntriesResponse(AppendEntriesReply),
}