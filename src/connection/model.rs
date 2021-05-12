extern crate reqwest;
extern crate anyhow;

#[derive(Debug, Clone)]
pub struct ChanConnection {
    pub client: reqwest::Client,
    pub lastpost: u32, // i really hope the 4294967295 will be enough lmao
    pub get_url: String,
    pub post_url: String,
    pub anna_cookie: String,
}


impl ChanConnection {
    pub async fn init(
        anna_cookie: String,
        get_url: String,
        post_url: String,
    ) -> Result<Self, anyhow::Error> {
        let client = reqwest::Client::builder().build()?;

        return Ok(Self {
            client: client,
            lastpost: 0u32,
            get_url: get_url,
            post_url: post_url,
            anna_cookie: anna_cookie,
        })
    }

    pub fn set_lastpost(&mut self, latest: u32){
        self.lastpost = latest;
    }
}