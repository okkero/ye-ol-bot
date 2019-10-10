use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
use serenity::model::prelude::ChannelId;
use crate::parser;
use crate::sorter::SortedCodes;

const RECEIVE_CHANNEL: u64 = 631565506998042626;
const SEND_CHANNEL: u64 = 631904101348933653;

struct Here;
impl Mentionable for Here {
    fn mention(&self) -> String {
        "@here".to_string()
    }
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.is_own(context.cache) {
            return;
        }

        println!();
        println!();
        println!();


        let ChannelId(channel_id) = msg.channel_id;
        println!("Message in channel {}...￿", channel_id);

        if channel_id == RECEIVE_CHANNEL {
            println!("Correct channel!");
            println!("Parsing codes");

            let parsed_codes = parser::parse_codes(&msg.content);
            if parsed_codes.is_empty() {
                println!("Found no codes");
                return;
            }

            println!("Codes found: {}", parsed_codes.len());
            let sorted_codes = SortedCodes::from_unsorted(parsed_codes);
            print_sorted_codes(&sorted_codes);

            let message = build_codes_message(&sorted_codes).build();
            if let Err(err) = ChannelId(SEND_CHANNEL).say(&context.http, &message) {
                println!("Error sending message: {:?}", err);
            }
        } else {
            println!("Incorrect channel :(");
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn start(token: &str) {
    let mut client = Client::new(token, Handler).expect("Err creating client");

    println!("Starting client...");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

fn print_sorted_codes(codes: &SortedCodes) {
    println!("======= Confident PC =======");
    for code in &codes.pc {
        println!("{:?}", code);
    }
    println!();
    println!("===== Confident not PC =====");
    for code in &codes.not_pc {
        println!("{:?}", code);
    }
    println!();
    println!("======== Uncertain =========");
    for code in &codes.uncertain {
        println!("{:?}", code);
    }
}

fn build_codes_message(codes: &SortedCodes) -> MessageBuilder {
    let mut builder = MessageBuilder::new();
    builder
        .mention(&Here)
        .push_line("");

    let has_pc_codes = !codes.pc.is_empty();
    let has_not_pc_codes = !codes.not_pc.is_empty();
    let has_uncertain_codes = !codes.uncertain.is_empty();

    if has_pc_codes {
        builder.push_line("I found some SHiFT codes for you. I am quite confident these are for PC:");
        for code in &codes.pc {
            builder.push_bold_line(&code.code);
        }
        builder.push_line("");
    }

    if has_uncertain_codes {
        if has_pc_codes {
            builder.push_line(
                "I also found the following codes, \
                but I could not figure out which platform they are for. Try them:"
            );
        } else {
            builder.push_line(
                "I found some SHiFT codes for you. \
                I could not figure out which platform they are for. Try them:");
        }
        for code in &codes.uncertain {
            builder.push_bold_line(&code.code);
        }
        builder.push_line("");
    }

    if has_not_pc_codes {
        if has_pc_codes || has_uncertain_codes {
            builder.push_line(
                "There were these, as well, but I am fairly certain they are not for PC. \
                You could, of course, be a pleb and redeem them anyway, if you really have to:"
            );
        } else {
            builder.push_line(
                "I found some SHiFT codes for you, but I am fairly certain they are not for PC. \
                You could, of course, be a pleb and redeem them anyway, if you really have to:"
            );
        }
        for code in &codes.not_pc {
            builder.push_bold_line(&code.code);
        }
        builder.push_line("");
    }

    builder
}