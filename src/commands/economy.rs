use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::Args;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::id::UserId;
use tuple_conv::RepeatedTuple;
use rand::thread_rng;
use rand::Rng;
use num_format::{Locale, ToFormattedString};
use crate::useful::*;



#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    sendmessage("pong!", ctx, msg).await;

    Ok(())
}

#[command]
#[aliases("bal")]
async fn balance(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let author = match args.rest() {
        "" => msg.author.id,
        _ => args.single::<UserId>().unwrap()
    };
    //sendmessage(args.rest(), ctx, msg).await;
    let _unused = check_if_registered(msg);
    let result = search_statement(format!("SELECT * FROM player WHERE id={}", author).as_str());
    let mut bal_message = String::new();
    let mut y = 0;
    for x in result.unwrap().materials {
        if x != 0 { match bal_message.as_str() {
            "" => bal_message = format!(":{}: {}", GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),
            _ => bal_message = format!("{}\n:{}: {}", bal_message, GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),}
        } y += 1;
    }
    if bal_message.as_str() == "" {
        bal_message = "You have Nothing...".to_string();
    }
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Balance", msg.author.name).as_str());
            e.description(format_emojis(bal_message.to_owned()).as_str());
            e.color(randcolor);
            e.author(|a| {
                a.icon_url(msg.author.avatar_url().unwrap());
                a.name(msg.author.name.as_str());
                a
            });e
        });m
    }).await {
        sendmessage(format!("Error {}", why).as_str(), ctx, msg).await;
    }

    Ok(())
}

#[group]
#[commands(ping, balance)]
struct Economy;