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

#[group]
#[commands(panic)]
#[owners_only]
pub struct Owner;