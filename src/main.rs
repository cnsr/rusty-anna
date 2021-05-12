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

    let url = format!("https://{}/last/{}/", domain, board);
    let post_url = format!("https://{}/chat/{}/", domain, board);
    
    let con = connection::ChanConnection::init(
        anna_cookie, url, post_url,
    ).await?;


    let greeting = message::OutboundMessage {
        chat: String::from("int"),
        name: Some(String::from("salobot")),
        trip: Some(String::from("test")),
        body: String::from("Connected to the chat."),
        convo: String::from("General"),
    };

    //  notify about successful connection
    // post_message(&con, greeting).await?;

    loop {
        get_messages(&con).await?;
    }

}

async fn get_messages(con: &connection::ChanConnection) -> Result<(), anyhow::Error> {
    let response = con.client
        .get(&con.get_url)
        .header("Set-Cookie", format!("password_livechan={}", con.anna_cookie))
        .send()
        .await?
        .text()
        .await?;
        
    let messages: Vec<message::InboundMessage> = serde_json::from_str(&response).unwrap();
        
    // println!("{:#?}", messages);
    println!("\n\n\nfirst: {:#?}", messages[0]);
    
    sleep(Duration::from_millis(250)).await;
    Ok(())
}


async fn post_message(con: &connection::ChanConnection, message: message::OutboundMessage) -> Result<(), anyhow::Error> {
    let serialized_message = serde_json::json!(&message);
    // let serialized_message = serde_json::to_value(&message)?;
    println!("data: {:#?}", &serialized_message);

    let response = con.client
        .post(&con.post_url)
        // .post("https://jsonplaceholder.typicode.com/posts")
        .header("Set-Cookie", format!("password_livechan={}", con.anna_cookie))
        // .header("Content-Type", "application/json")
        // .form(&serialized_message)
        .form(&serialized_message)
        .send()
        .await?
        .text()
        .await?;

    // let json_response: serde_json::Value = serde_json::from_str(&response)?;
    println!("response: {:#?}", response);

    // the response should be {"success":"success_posting","id":5171570}
    // what i am getting is "{\"failure\":\"database_update_error\"}"
    
    Ok(())
}
