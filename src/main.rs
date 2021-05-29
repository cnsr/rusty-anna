use std::env;
// use std::path::Path;
use tokio::time::{sleep, Duration};

extern crate dotenv;
extern crate reqwest;
extern crate serde_json;
extern crate anyhow;

// local modules
mod connection;
mod message;
mod commands;

// external crates
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    
    // anna_nolimit_cookie1
    let anna_cookie = env::var("ANNA_COOKIE")
        .expect("ANNA_COOKIE is not set in the .env file");
    // let anna_cookie = "";

    // kotchan.fun
    let domain = env::var("DOMAIN")
        .expect("DOMAIN is not set in the .env file");

    let board = env::var("BOARD")
        .expect("BOARD is not set in the .env file");

    let name = env::var("NAME")
        .expect("NAME is not set in the .env file");

    // FIXME: tripcode doesnt actually do anything - need to figure out why
    let trip = env::var("TRIP")
        .expect("TRIP is not set in the .env file");

    let accept_invalid_certs = env::var("ACCEPT_INVALID_CERTS")
        .expect("ACCEPT_INVALID_CERTS is not set in the .env file")
        .into_bool();

    let get_url = format!("https://{}/last/{}/", domain, board);
    let post_url = format!("https://{}/chat/{}/", domain, board);
    
    let mut con = connection::ChanConnection::init(
        accept_invalid_certs,
        anna_cookie,
        get_url, post_url,
        name, trip
    ).await?;
    
    // TODO: remove?
    let _greeting = message::OutboundMessage {
        chat: String::from("int"),
        name: Some(String::from("salobot")),
        trip: Some(String::from("test")),
        body: String::from("Connected to the chat."),
        convo: String::from("GeneralDEBUG"),
        reply_to: None,
    };
    con.add_to_outbound_queue(_greeting).await?;

    loop {
        // retrieve the latest messages
        con.get_and_process_messages().await?;

        // notify about successful connection
        con.attempt_sending_outbound().await?;

        // timeout should be at the very least 1 second between running the loop cycles
        // TODO: cleanup
        sleep(Duration::from_millis(1000)).await;
    }
}

trait IntoBool {
    fn into_bool(self) -> bool;
}

impl IntoBool for String {
    fn into_bool(self) -> bool {
        // add any values here
        let possible_boolean_trues = vec![
            String::from("1"),
            String::from("true"),
            String::from("True"),
        ];
        return possible_boolean_trues.contains(&self);
    }
}