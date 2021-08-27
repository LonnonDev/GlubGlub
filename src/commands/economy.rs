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

use crate::format_emojis;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    sendmessage("pong!", ctx, msg).await;

    Ok(())
}

#[command]
#[aliases("info")]
async fn information(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get author id
    let author_id = args.single::<UserId>().unwrap_or(msg.author.id);
    let author = msg.guild_id.unwrap().member(ctx, author_id).await.unwrap().user;

    // Registers the user if they don't exist
    let _ = check_if_registered(*author_id.as_u64());

    // Get the players grist
    let result = get_player(*author_id.as_u64());

    // Put all of the grist the user has in a string
    let mut info_message = String::new();
    let mut y = 0;
    for x in result.unwrap().materials {
        if x != 0 { match info_message.as_str() {
            "" => info_message = format!(":{}: {}", GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),
            _  => info_message = format!("{}\n:{}: {}", info_message, GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),}
        } y += 1;
    }
    if info_message.as_str() == "" {
        info_message = "You have Nothing...".to_string();
    }

    // Random color for embed and send embed
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Player info", author.name).as_str());
            e.description(format_emojis!("{}", info_message));
            e.color(randcolor);
            e.author(|a| {
                a.icon_url(author.avatar_url().unwrap());
                a.name(author.name.as_str());
                a
            });e
        });m
    }).await {
        sendmessage(format!("Error {}", why).as_str(), ctx, msg).await;
    }

    Ok(())
}

#[command]
#[bucket("basic")]
async fn game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get author id
    let author_id = args.single::<UserId>().unwrap_or(msg.author.id);
    let author = msg.guild_id.unwrap().member(ctx, author_id).await.unwrap().user;

    // Registers the user if they don't exist
    let _ = check_if_registered(*author_id.as_u64());

    // Random message for the embed
    let randnum: u8 = thread_rng().gen_range(0..=6);
    let randommessage: &str = match randnum {
        0 => "You ripped some puppet ass",
        1 => "You killed some imps",
        2 => "You played the market",
        3 => "You torrented some grist",
        4 => "You cascaded some monsters",
        5 => "You gained some ranks",
        6 => "You caused **the** scratch",
        _ => "how the fuck did you get here?"
    };

    // Random Amount of Grist
    let random_grist: i64 = thread_rng().gen_range(1..30);

    // Get the player
    let result = get_player(*author_id.as_u64());
    let player = result.unwrap();

    // Get the new grist value, and update the player
    let newvalue = random_grist + player.materials.build;
    let _ = sqlstatement(format!("UPDATE player SET build={} WHERE id={}", newvalue, author.id.as_u64()).as_str());

    // Send exile quote
    get_exile_quote(ctx, msg).await;

    // Random color for embed and Send embed
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(randommessage);
            e.description(format_emojis!("You got :build: {}", random_grist));
            e.image("https://media1.tenor.com/images/7d27136cbf1967f8f5d3f0481b3a8c38/tenor.gif");
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
 
//TODO Implement Achlemizing
#[command]
async fn craft(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    sendmessage(args.rest(), ctx, msg).await;
    Ok(())
}

#[group]
#[only_in("guilds")]
#[commands(ping, information, game)]
pub struct Economy;