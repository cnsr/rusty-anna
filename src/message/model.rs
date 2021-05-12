use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OutboundMessage {
    pub chat: String,
    pub name: Option<String>,
    pub trip: Option<String>,
    pub body: String,
    pub convo: String,
}


// TODO: add images
/*
thumb
image_height
image_width
image_filesize
image_filename
image
*/
#[derive(Debug, Deserialize)]
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
}