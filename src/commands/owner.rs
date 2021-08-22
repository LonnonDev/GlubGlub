use rand::Rng;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use std::process;
use crate::useful::*;

//* Panic command
//* Stops the bot
#[command]
async fn panic(ctx: &Context, msg: &Message) -> CommandResult {
    sendmessage("Stopping the Bot...", ctx, msg).await;
    println!("Bluh `panic` command was ran\nStopping Bot...");
    process::exit(0);
}


#[command]
async fn author_exile_test(ctx: &Context, msg: &Message) -> CommandResult {
    let mut ids = vec![];
    for _ in 0..10 {
        let rand_id = rand::thread_rng().gen_range(111111111111111111u128..999999999999999999u128);
        ids.push(rand_id);
        ids.push(rand_id % 4);
    }
    sendmessage(format!("{:#?}", ids).as_str(), ctx, msg).await;
    Ok(())
}

#[group]
#[commands(panic, author_exile_test)]
#[only_in("guilds")]
#[owners_only]
pub struct Owner;