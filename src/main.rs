use crate::sorter::SortedCodes;
use std::env;
use std::error::Error;

mod parser;
mod data;
mod sorter;
mod bot;

fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("YE_OL_BOT_TOKEN")?;
    let receive_channel = env::var("YE_OL_BOT_RECEIVE")?.parse::<u64>()?;
    let send_channel = env::var("YE_OL_BOT_SEND")?.parse::<u64>()?;
    bot::start(&token, receive_channel, send_channel);

    Ok(())
}
