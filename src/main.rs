use crate::sorter::SortedCodes;
use std::env;
use std::error::Error;

mod parser;
mod data;
mod sorter;
mod bot;

fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("YE_OL_BOT_TOKEN")?;
    bot::start(&token);

    Ok(())
}
