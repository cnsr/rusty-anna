extern crate reqwest;
extern crate anyhow;

use crate::message::{InboundMessage, MessageQueue, OutboundMessage};
use http::{HeaderMap, HeaderValue, header::{COOKIE}};


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

    pub fn construct_reply_text(&self, text: String) -> String {
        return format!(">>{}\n{}", self.lastpost, text);
    }

    pub async fn add_to_queue(&mut self, message: InboundMessage) -> Result<(), anyhow::Error> {
        //  TODO: check for messages in the outbouund history
        let is_bot = false;
        self.lastpost = message.count;
        self.queue.add_to_queue(message, is_bot).await?;
        Ok(())
    }

    // pub fn construct_reply(&self, message: InboundMessage) -> OutboundMessage {
    //     return OutboundMessage {
    //         chat: message.chat,
    //         pub name: Option<String>,
    //         pub trip: Option<String>,
    //         pub body: String,
    //         pub convo: String,
    //     };
    // }

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
}