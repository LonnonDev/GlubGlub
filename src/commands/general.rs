use core::time;
use std::thread;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use rand::{Rng, thread_rng};
use serenity::model::id::UserId;
use serenity::prelude::Mentionable;
use crate::useful::*;

#[check]
#[name = "SoulFlame"]
async fn soul_flame(
    _: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    // Replace 7 with your ID to make this check pass.
    //
    // 1. If you want to pass a reason alongside failure you can do:
    // `Reason::User("Lacked admin permission.".to_string())`,
    //
    // 2. If you want to mark it as something you want to log only:
    // `Reason::Log("User lacked admin permission.".to_string())`,
    //
    // 3. If the check's failure origin is unknown you can mark it as such:
    // `Reason::Unknown`
    //
    // 4. If you want log for your system and for the user, use:
    // `Reason::UserAndLog { user, log }`
    if *msg.guild_id.unwrap().as_u64() != 785241980162408450u64 {
        return Err(Reason::User("Not Correct Guild".to_string()));
    }

    Ok(())
}

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

#[command]
#[aliases("ring")]
#[bucket("ring")]
async fn call(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let authorid = args.single::<UserId>().unwrap_or(msg.author.id);
    let mut amount = args.single::<u32>().unwrap_or(5u32);
    if amount > 20 {
        amount = 20;
    } else if amount < 1 {
        amount = 1;
    }
    let author = msg.guild_id.unwrap().member(ctx, authorid).await.unwrap().user;
    for _i in 0..amount {
        sendmessage(format!("{} Ring Ring...", author.mention().to_string().as_str()).as_str(), ctx, msg).await;
        let timeout_time = time::Duration::from_secs(3);
        thread::sleep(timeout_time);
    }

    Ok(())
}

#[group]
#[only_in("guilds")]
#[commands(info, call)]
pub struct General;