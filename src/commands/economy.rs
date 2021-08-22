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
    let authorid = args.single::<UserId>().unwrap_or(msg.author.id);
    let author = msg.guild_id.unwrap().member(ctx, authorid).await.unwrap().user;
    //sendmessage(args.rest(), ctx, msg).await;
    let _unused = check_if_registered(msg);
    let result = search_statement(format!("SELECT * FROM player WHERE id={}", authorid).as_str());
    let mut bal_message = String::new();
    let mut y = 0;
    for x in result.unwrap().materials {
        if x != 0 { match bal_message.as_str() {
            "" => bal_message = format!(":{}: {}", GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),
            _  => bal_message = format!("{}\n:{}: {}", bal_message, GRIST_TYPES.to_vec()[y], x.to_formatted_string(&Locale::en)),}
        } y += 1;
    }
    if bal_message.as_str() == "" {
        bal_message = "You have Nothing...".to_string();
    }
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Balance", author.name).as_str());
            e.description(format_emojis(bal_message.to_owned()).as_str());
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
async fn game(ctx: &Context, msg: &Message) -> CommandResult {
    let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
    let randnum: u8 = thread_rng().gen_range(0..=5);
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
    let randnum2: i64 = thread_rng().gen_range(1..30);
    let result = search_statement(format!("SELECT * FROM player WHERE id={}", msg.author.id.as_u64()).as_str());
    let player = result.unwrap();
    let newvalue = randnum2 + player.materials.build;
    let _ = sqlstatement(format!("UPDATE player SET build={} WHERE id={}", newvalue, msg.author.id.as_u64()).as_str());
    get_exile_quote(ctx, msg).await;
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(randommessage);
            e.description(format_emojis(format!("{} and got :build: {}", randommessage, randnum2)).as_str());
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

#[command]
async fn craft(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    sendmessage(args.rest(), ctx, msg).await;
    Ok(())
}

#[group]
#[only_in("guilds")]
#[commands(ping, balance, game)]
pub struct Economy;