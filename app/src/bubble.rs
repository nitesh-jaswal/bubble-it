use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub const MAX_TEXT_LENGTH: usize = 500;


pub struct Message {
    text: [char; MAX_TEXT_LENGTH],
    length: usize
}

impl FromStr for Message {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text: Result<[char; MAX_TEXT_LENGTH], _> = s.chars().collect::<Vec<char>>().try_into();
        match text {
            Ok(t) => return Ok(Self {text: t, length: t.len() }),
            Err(e) =>  return Err(format!("ERROR: Message parsing failed => {:?}", e))
        }
    }
}


pub struct Bubblett {
    sender: String,
    sequence: usize,
    message: Message,
    received_timestamp: DateTime<Utc> // Q: Does it make sense to have sent_timestamp?
}

impl Bubblett {
    pub fn new(sender: String, m: Message, s: usize) -> Self {
        Self { sender: sender, sequence: s , message: m, received_timestamp: Utc::now() }
    }
}

pub struct Bubble {
    pub id: Uuid,
    pub display_name: String, // TODO: Length limit here
    permanent_name: String, // Q: Custom type here?
    bubbletts: Vec<Bubblett>,
    pub last_message_sent: Option<DateTime<Utc>>,
    pub creator: String,
    members: Vec<String>,
    pub active_sequence: usize
}

impl Bubble {
    pub fn new(creator: String, display_name: &str, members: Vec<String>) -> Self {
        Self { 
            id: Uuid::new_v4(), 
            display_name: display_name.to_string(), 
            permanent_name: "Test-Panda-nAM0hNamAH".to_string(),
            bubbletts: Vec::new(), 
            last_message_sent: None, 
            creator: creator,
            members: members ,
            active_sequence: 0usize
        }
    }
    

    fn add_bubblett(&mut self, bubblett: Bubblett) -> Result<(), String> {
        if !self.members.contains(&bubblett.sender) {
            return Err("ERROR: Member cannot participate in the bubble".to_string())
        }
        self.last_message_sent = Some(bubblett.received_timestamp);
        self.active_sequence = bubblett.sequence;
        self.bubbletts.push(bubblett);
        Ok(())
    }
    // 💡: Request delete of bubblett on other member approval
    
}
