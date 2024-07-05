use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        token: discord_token()
    };
}

pub fn discord_token() -> String {
    dotenv().ok();
    var("TOKEN").expect("Discord Token is not set")
}
