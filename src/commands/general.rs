use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use rand::{Rng, thread_rng};
use crate::useful::*;


#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("This is a Bot made in Rust").as_str());
            e.description(format_emojis("This bot is inspired off of homestuck :build:
            This bot is made in the serenity rust api wrapper for discord .
            ".to_string()).as_str());
            e.color(randcolor);
            e
        });m
    }).await {
        sendmessage(format!("Error {}", why).as_str(), ctx, msg).await;
    }

    Ok(())
}

#[group]
#[commands(info)]
struct General;