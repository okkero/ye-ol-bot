use crate::sorter::SortedCodes;

mod parser;
mod data;
mod sorter;
mod bot;

fn main() {
    bot::start();
}
