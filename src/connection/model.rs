extern crate reqwest;
extern crate anyhow;
extern crate serde_json;

// external
use http::{HeaderMap, HeaderValue, header::{COOKIE}};

// local
use crate::message::{InboundMessage, MessageQueue, OutboundMessage, PostResult};


struct BotConfiguration {
    pub name: String,
    pub trip: String
}

// impl BotConfiguration {
//     pub async fn init() -> Result<Self, anyhow::Error> {

//     }
// }

#[derive(Debug)]
pub struct ChanConnection {
    pub client: reqwest::Client,
    pub lastpost: u32, // i really hope the 4294967295 will be enough lmao
    limit: u8,
    raw_get_url: String,
    queue: MessageQueue,
    pub post_url: String,
    pub anna_cookie: String,
    /*
        TODO: implement a way to store a set of outbound messages (as InboundMessage)
        Would be great for the API to properly function first i guess else it's gonna be fugly
    */
}


impl ChanConnection {
    pub async fn init(
        anna_cookie: String,
        get_url: String,
        post_url: String,
    ) -> Result<Self, anyhow::Error> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()?;
        let queue = MessageQueue::init().await?;

        return Ok(Self {
            client: client,
            lastpost: 0u32,
            limit: 1u8,
            queue: queue,
            raw_get_url: get_url,
            post_url: post_url,
            anna_cookie: anna_cookie,
        })
    }

    pub fn set_lastpost(&mut self, latest: u32) {
        self.lastpost = latest;
    }

    // why cant this be something like pyhton @property tho
    pub fn get_url(&self) -> String {
        let mut result = format!("{}?count={}", self.raw_get_url, self.limit);
        if self.lastpost != 0u32 {
            result = format!("{}?count={}", result, self.lastpost);
        }
        println!("{}", result);
        return result;
    }

    pub fn construct_reply_text(&self, text: String, to: Option<u32>) -> String {
        let postnumber = match to {
            Some(number) => number,
            None => self.lastpost
        };
        return format!(">>{}\n{}", postnumber, text);
    }

    pub async fn add_to_queue(&mut self, message: InboundMessage) -> Result<(), anyhow::Error> {
        //  TODO: check for messages in the outbouund history
        let is_bot = self.queue.check_if_outbound(message.clone()).await?;
        self.lastpost = message.count;
        self.queue.add_to_queue(message, is_bot).await?;
        Ok(())
    }

    // TODO: add a config to have where to pull the variables from
    pub fn construct_reply(&self, message: InboundMessage) -> OutboundMessage {
        // construct a reply for an outbound message
        return OutboundMessage {
            chat: message.chat,
            name: None,
            trip: None,
            body: self.construct_reply_text(String::from("Reply"), Some(message.count)),
            convo: message.convo,
        };
    }

    pub async fn process_messages(&self, messages: Vec<InboundMessage>) -> Result<(), anyhow::Error> {
        // TODO: implement
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

    pub async fn send_message(&self, message: OutboundMessage) -> Result<bool, anyhow::Error> {
        let serialized_message = serde_json::json!(&message);

        let response = self.client
            .post(&self.post_url)
            // .post("https://jsonplaceholder.typicode.com/posts")
            .headers(self.headers())
            // .form(&serialized_message)
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
                let result: bool = self.send_message(message.clone()).await?;
                match result {
                    false => {
                        self.queue.append_as_first(message);
                    },
                    _ => return Ok(())
                }
                return Ok(());
            },
            None => {
                return Ok(());
            }
        }
    }
}