// #![feature(in_band_lifetimes)]

use futures::future::Either;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Debug, Clone)]
pub struct OutboundMessage {
    pub chat: String,
    pub name: Option<String>,
    pub trip: Option<String>,
    pub body: String,
    pub convo: String,
    pub reply_to: Option<u32>
}

// known failures: "database_update_error", "countdown_violation"
// known successes: "success_posting"
#[derive(Deserialize, Debug)]
pub struct PostResult {
    pub success: Option<String>,
    pub failure: Option<String>,
    pub id: Option<u32>,
}

impl PostResult {
    pub fn is_successful(&self) -> bool {
        return self.success != None;
    }
    pub fn failed_to_send(&self) -> bool {
        println!("Checking PostResult: {:#?}", self);
        match &self.failure {
            Some(reason) => reason == "countdown_violation",
            None => false
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct InboundMessage {
    pub _id: String,
    pub body: String,
    pub chat: String,
    pub convo: String,
    pub count: u32,
    // in some rare cases the country value is missing
    pub country: Option<String>,
    pub country_name: Option<String>,
    pub date: String,
    pub identifier: String,
    pub name: String,
    pub trip: Option<String>,
    pub thumb: Option<String>,
    pub image_height: Option<u16>,
    pub image_width: Option<u16>,
    pub image_filesize: Option<u32>,
    pub image_filename: Option<String>,
    pub image: Option<String>,
    pub duration: Option<f32>,
    pub replied_to: Option<bool>,
}

impl PartialEq for InboundMessage {
    fn eq(&self, other: &Self) -> bool {
        return self.count == other.count;
    }
}

impl PartialEq for OutboundMessage {
    fn eq(&self, other: &Self) -> bool {
        return self.convo == other.convo && self.reply_to == other.reply_to;
    }
}

// compare InboundMessage to Outbound message
impl PartialEq<OutboundMessage> for InboundMessage {
    fn eq(&self, other: &OutboundMessage) -> bool {
        // not sure how else to compare
        return self.body == other.body && self.convo == other.convo;
    }
}

// same but other way around
impl PartialEq<InboundMessage> for OutboundMessage {
    fn eq(&self, other: &InboundMessage) -> bool {
        // not sure how else to compare
        return self.body == other.body && self.convo == other.convo;
    }
}

#[derive(Debug)]
pub struct MessageQueue {
    messages: Vec<InboundMessage>,
    bot_messages: Vec<InboundMessage>,
    // this is gonna be the post queue for bot replies - sent every 3 seconds or so
    outbound_messages: Vec<OutboundMessage>,
    outbound_messages_history: Vec<OutboundMessage>,
    limit_messages: u8,
    limit_bot_messages: u8,
    limit_outbound_messages: u8,
}

trait FirstPoppable {
    fn pop_first(&mut self) -> Option<OutboundMessage>;
    fn insert_as_first(&mut self, first_item: OutboundMessage) -> Result<(), anyhow::Error>;
}

impl FirstPoppable for Vec<OutboundMessage> {
    fn pop_first(&mut self) -> Option<OutboundMessage> {
        match self.len() > 0 {
            true => {
                let first_item = self.remove(0);
                Some(first_item)
            },
            false => None
        }
    }

    fn insert_as_first(&mut self, first_item: OutboundMessage) -> Result<(), anyhow::Error> {
        self.insert(0, first_item);
        Ok(())
    }
}

trait MarkAsRepliedTo {
    fn mark(&mut self, message_id: u32);
}

impl MarkAsRepliedTo for Vec<InboundMessage> {
    fn mark(&mut self, message_id: u32) {
        for message in self {
            if message.count == message_id {
                message.replied_to = Some(true);
            }
        }
    }

}

impl MessageQueue {
    //  TODO: LIMITs from .env
    pub async fn init() -> Result<Self> {
        let messages = Vec::new();
        let bot_messages = Vec::new();
        let outbound_messages = Vec::new();
        let outbound_messages_history = Vec::new();
        return Ok(Self {
            messages: messages,
            bot_messages: bot_messages,
            outbound_messages: outbound_messages,
            outbound_messages_history: outbound_messages_history,
            limit_messages: 20u8,
            limit_bot_messages: 30u8,
            limit_outbound_messages: 20u8,
        });
    }

    pub fn first_to_send(&mut self) -> Option<OutboundMessage> {
        self.outbound_messages.pop_first()
    }

    pub fn append_as_first(&mut self, first_item: OutboundMessage) {
        self.outbound_messages.insert_as_first(first_item).unwrap();
    }

    pub async fn add_to_outbound_queue(&mut self, message: OutboundMessage) -> Result<(), anyhow::Error> {
        self.outbound_messages.push(message);
        Ok(())
    }

    pub async fn mark_as_replied_to(&mut self, reply_to: u32) -> Result<(), anyhow::Error> {
        if reply_to != 0u32 { self.messages.mark(reply_to); }
        Ok(())
    }

    pub fn contains(&self, message: OutboundMessage) -> bool {
        return self.outbound_messages.contains(&message) || self.outbound_messages_history.contains(&message);
    }

    pub async fn add_to_queue(&mut self, message: InboundMessage, is_bot: bool) -> Result<bool, anyhow::Error> {
        match is_bot {
            true => {
                // contains should re-implemented to check by postcount?
                if !self.bot_messages.contains(&message) {
                    // only stores 20 latest bot message - should probably be longer
                    if self.bot_messages.len() > 20 as usize {
                        self.bot_messages.remove(0);
                    }
                    self.bot_messages.push(message);
                    return Ok(true);
                }
            },
            false => {
                if !self.messages.contains(&message) {
                    // only store 50 latest message - maybe even less? dont need to have that many messages saved lol
                    if self.messages.len() > 20 as usize {
                        self.messages.remove(0);
                    }
                    self.messages.push(message);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub async fn add_to_history(&mut self, message: InboundMessage) -> Result<(), anyhow::Error> {
        self.messages.push(message);
        Ok(())
    }

    pub async fn add_to_outbound_history(&mut self, message: OutboundMessage) -> Result<(), anyhow::Error> {
        self.outbound_messages_history.push(message);
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<(), anyhow::Error> {
        // this might be very bad
        if self.messages.len() > 0 {
            while self.messages.len() as u8 > self.limit_messages {
                self.messages.remove(0);
            }
        }
        if self.bot_messages.len() > 0 {
            while self.bot_messages.len() as u8 > self.limit_bot_messages {
                self.bot_messages.remove(0);
            }
        }
        if self.outbound_messages_history.len() > 0 {
            while self.outbound_messages_history.len() as u8 > self.limit_outbound_messages {
                self.outbound_messages_history.remove(0);
            }
        }
        Ok(())
    }

    pub async fn check_if_outbound(&mut self, message: InboundMessage) -> Result<bool, anyhow::Error> {
        for outbound_message in &self.outbound_messages_history {
            if outbound_message.to_owned() == message {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
