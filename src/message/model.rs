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
}


/* 
An example of inbound JSON (with video attached)
Object({
    "_id": String(
        "609ec7f86581548b88e3d055",
    ),
    "body": String(
        "",
    ),
    "chat": String(
        "int",
    ),
    "convo": String(
        "General",
    ),
    "count": Number(
        5177237,
    ),
    "country": String(
        "US-IL",
    ),
    "country_name": String(
        "United States",
    ),
    "date": String(
        "2021-05-14T18:56:56.000Z",
    ),
    "duration": Number(
        9.543,
    ),
    "identifier": String(
        "$2a$10$mAM0oYrjp0bCHFDsGaiB.e4H1m/Rz/MYV4RHlSeIZtmOxBNiVt7dm",
    ),
    "image": String(
        "/home/ph/livechan-js/public/tmp/uploads/24244-16iz49f.mp4",
    ),
    "image_filename": String(
        "Squilliam sings shawty like a melody.mp4",
    ),
    "image_filesize": Number(
        333649,
    ),
    "image_height": Number(
        302,
    ),
    "image_width": Number(
        480,
    ),
    "name": String(
        "Kot",
    ),
    "thumb": String(
        "/home/ph/livechan-js/public/tmp/thumb/24244-16iz49f.jpg",
    ),
}),
*/

#[derive(Debug, Deserialize, Clone)]
pub struct InboundMessage {
    pub _id: String,
    pub body: String,
    pub chat: String,
    pub convo: String,
    pub count: u32,
    pub country: String,
    pub country_name: String,
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
}

impl PartialEq for InboundMessage {
    fn eq(&self, other: &Self) -> bool {
        return self.count == other.count;
    }
}

#[derive(Debug)]
pub struct MessageQueue {
    messages: Vec<InboundMessage>,
    bot_messages: Vec<InboundMessage>,
    // this is gonna be the post queue for bot replies - sent every 3 seconds or so
    outbound_messages: Vec<OutboundMessage>,
}

trait FirstPoppable {
    fn pop_first(&mut self) -> Result<OutboundMessage, anyhow::Error>;
}

impl FirstPoppable for Vec<OutboundMessage> {
    fn pop_first(&mut self) -> Result<OutboundMessage, anyhow::Error> {
        let first_item = self.remove(0);
        Ok(first_item)
    }
}

impl MessageQueue {
    pub async fn init() -> Result<Self> {
        let mut messages = Vec::new();
        let mut bot_messages = Vec::new();
        let mut outbound_messages = Vec::new();
        return Ok(Self {
            messages: messages,
            bot_messages: bot_messages,
            outbound_messages: outbound_messages,
        });
    }

    pub async fn add_to_queue(&mut self, message: InboundMessage, is_bot: bool) -> Result<()> {
        match is_bot {
            true => {
                // contains should re-implemented to check by postcount?
                if !self.bot_messages.contains(&message) {
                    // only stores 20 latest bot message - should probably be longer
                    if self.bot_messages.len() > 20 as usize {
                        self.bot_messages.remove(0);
                    }
                    self.bot_messages.push(message);
                }
            },
            false => {
                if !self.messages.contains(&message) {
                    // only store 20 latest message - maybe even less? dont need to have that many messages saved lol
                    if self.messages.len() > 20 as usize {
                        self.messages.remove(0);
                    }
                    self.messages.push(message);
                }
            }
        }
        Ok(())
    }
}