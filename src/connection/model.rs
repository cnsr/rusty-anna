extern crate reqwest;
extern crate anyhow;
extern crate serde_json;

// external
use http::{HeaderMap, HeaderValue, header::{COOKIE}};

// local
use crate::message::{InboundMessage, MessageQueue, OutboundMessage, PostResult};
use crate::commands::{Command, CommandSet};

#[derive(Debug)]
struct BotConfiguration {
    pub name: String,
    pub trip: String
}

impl BotConfiguration {
    pub async fn init(name: String, trip: String) -> Result<Self, anyhow::Error> {
        Ok(Self {
            name: name,
            trip: trip,
        })
    }
}

#[derive(Debug)]
pub struct ChanConnection {
    pub client: reqwest::Client,
    config: BotConfiguration,
    pub lastpost: u32, // i really hope the 4294967295 will be enough lmao
    limit: u8,
    raw_get_url: String,
    queue: MessageQueue,
    pub post_url: String,
    pub anna_cookie: String,
    commands: CommandSet,
}


impl ChanConnection {
    pub async fn init(
        accept_invalid_certs: bool,
        anna_cookie: String,
        get_url: String,
        post_url: String,
        name: String,
        trip: String,
    ) -> Result<Self, anyhow::Error> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(accept_invalid_certs)
            .cookie_store(true)
            .build()?;

        let queue = MessageQueue::init().await?;

        let config = BotConfiguration::init(name, trip).await?;

        let commands = CommandSet::init().await?;

        return Ok(Self {
            client: client,
            config: config,
            lastpost: 0u32,
            limit: 5u8, // 1u8 doesn't seem to be working like I initially intended
            queue: queue,
            raw_get_url: get_url,
            post_url: post_url,
            anna_cookie: anna_cookie,
            commands: commands,
        })
    }

    // why cant this be something like pyhton @property tho
    pub fn get_url(&self) -> String {
        let mut result = format!("{}?limit={}", self.raw_get_url, self.limit);
        if self.lastpost != 0u32 {
            result = format!("{}&count={}", result, self.lastpost);
        }
        println!("Retrieving get_url: {:#?} {:#?} {:#?}", self.raw_get_url, self.limit, self.lastpost);
        println!("Result url: {}", result);
        return result;
    }

    pub fn construct_reply_text(&self, text: String, to: Option<u32>) -> String {
        let postnumber = match to {
            Some(number) => number,
            None => self.lastpost
        };
        return format!(">>{}\n{}", postnumber, text);
    }

    pub async fn add_to_queue(&mut self, mut message: InboundMessage) -> Result<(), anyhow::Error> {
        let is_bot = self.queue.check_if_outbound(message.clone()).await?;
        self.lastpost = message.count;
        let added_to_queue = self.queue.add_to_queue(message.clone(), is_bot).await?;
        if added_to_queue {
            println!("Added a message to the queue: {:#?}", message.count);
            println!("count: {:#?}\t is_bot: {:#?}\t is_replied_to {:#?}", message.count, is_bot, message.replied_to);
            if !is_bot && message.replied_to != Some(true) {
                match self.commands.check_against_commands(message.clone().body) {
                    Some (reply_text) => {
                        // message.replied_to = Some(true);
                        let new_message = self.construct_reply(message, reply_text);
                        self.add_to_outbound_queue(new_message).await?;
                        return Ok(());
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn construct_reply(&self, message: InboundMessage, raw_text: String) -> OutboundMessage {
        // construct a reply for an outbound message
        return OutboundMessage {
            chat: message.chat,
            name: Some(self.config.name.clone()),
            trip: Some(self.config.trip.clone()),
            body: self.construct_reply_text(raw_text, Some(message.count)),
            convo: message.convo,
            reply_to: Some(message.count),
        };
    }

    pub async fn process_messages(&mut self, mut messages: Vec<InboundMessage>) -> Result<(), anyhow::Error> {
        messages.reverse();
        for message in messages {
            self.add_to_queue(message).await?;
        }
        Ok(())
    }

    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers
            .insert(
                COOKIE,
                HeaderValue::from_str(&format!("password_livechan={}", self.anna_cookie)).unwrap()
            );

        headers
            .insert(
                COOKIE,
                HeaderValue::from_str(&format!("nolimitcookie={}", self.anna_cookie)).unwrap()
            );
        return headers;
    }

    pub async fn add_to_outbound_queue(&mut self, message: OutboundMessage) -> Result<(), anyhow::Error> {
        if !self.queue.contains(message.clone()) {
            self.queue.add_to_outbound_queue(message).await?;
        }
        Ok(())
    }

    pub async fn send_message(&self, message: OutboundMessage) -> Result<bool, anyhow::Error> {
        let serialized_message = serde_json::json!(&message);

        let response = self.client
            .post(&self.post_url)
            .headers(self.headers())
            .json(&serialized_message)
            .send()
            .await?
            .text()
            .await?;

        let post_result: PostResult = serde_json::from_str(&response)?;
        Ok(post_result.failed_to_send())
    }

    pub async fn attempt_sending_outbound(&mut self) -> Result<(), anyhow::Error> {
        match self.queue.first_to_send() {
            Some(message) => {
                println!("Sending out: {:?}", message);
                let result: bool = self.send_message(message.clone()).await?;
                println!("Sending out status: {:?}", result);
                match result {
                    true => {
                        self.queue.append_as_first(message);
                    },
                    false => {
                        if message.reply_to != None {
                            self.queue.mark_as_replied_to(message.reply_to.unwrap_or(0u32)).await?;
                        }
                        self.queue.add_to_outbound_history(message).await?;
                        return Ok(());
                    }
                }
                return Ok(());
            },
            None => {
                return Ok(());
            }
        }
    }

    pub async fn get_and_process_messages(&mut self) -> Result<(), anyhow::Error> {
        let response = &self.client
            .get(&self.get_url())
            .headers(self.headers())
            .send()
            .await?
            .text()
            .await?;
            
        let messages: Vec<InboundMessage> = serde_json::from_str(&response).unwrap();
        // println!("Messages: \n{:#?}", messages.clone());

        self.process_messages(messages).await?;
        Ok(())
    }
}