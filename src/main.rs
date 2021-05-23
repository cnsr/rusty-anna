use std::{env, os::linux::raw};
// use std::path::Path;
use tokio::time::{sleep, Duration};

extern crate dotenv;
extern crate reqwest;
extern crate serde_json;
extern crate anyhow;

// local modules
mod connection;
mod message;

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

    let get_url = format!("https://{}/last/{}/", domain, board);
    let post_url = format!("https://{}/chat/{}/", domain, board);
    
    // TODO: load chat, name and trip from env variables and save in the connection

    let mut con = connection::ChanConnection::init(
        anna_cookie, get_url, post_url, name, trip
    ).await?;
    
    loop {
        // retrieve the latest messages
        con.get_and_process_messages().await?;

        // this is temporary
        let _greeting = message::OutboundMessage {
            chat: String::from("int"),
            name: Some(String::from("salobot")),
            trip: Some(String::from("test")),
            body: String::from("Connected to the chat."),
            convo: String::from("GeneralDEBUG"),
        };

        //  notify about successful connection
        con.add_to_outbound_queue(_greeting).await?;
        con.attempt_sending_outbound().await?;

        // timeout should be at the very least 1 second between running the loop cycles
        sleep(Duration::from_millis(1000)).await;
    }
}