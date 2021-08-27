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
    let randnum: u8 = thread_rng().gen_range(0..=2);
    let randommessage: &str = match randnum {
        0 => "You ripped some puppet ass",
        1 => "You killed some imps",
        2 => "You gained some ranks",
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

trait InVec {
    fn in_vec(&self, vector: Vec<&str>) -> bool;
}

impl InVec for str {
    fn in_vec(&self, vector: Vec<&str>) -> bool {
        vector.contains(&self)
    }
}

trait VecStrToString<T> {
    fn vec_to_string(vector: Vec<T>) -> Vec<String>;
}

impl<T, S> VecStrToString<T> for Vec<S> where T: std::fmt::Display {
    fn vec_to_string(vector: Vec<T>) -> Vec<String> {
        let mut return_vector = vec![];
        for x in 0..vector.len() {
            return_vector.push(vector[x].to_string());
        }
        return return_vector;
    }
}

#[command]
async fn set_classpect(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let classes = vec!["Bard", "Prince", "Heir", "Page", "Seer", "Maid", "Sylph", "Knight", "Knight", "Witch", "Mage"];
    let aspects = vec!["Space", "Time", "Light", "Void", "Heart", "Mind", "Hope", "Rage", "Life", "Doom", "Breath", "Blood"];

    let author_id = *msg.author.id.as_u64();
    let classpect = vec![args.single::<String>().unwrap(), args.single::<String>().unwrap()];

    if classpect[0].in_vec(classes) && classpect[1].in_vec(aspects) {
        let _ = sqlstatement(format!("UPDATE player SET class={:?}, aspect={:?} WHERE id={}", classpect[0], classpect[1], author_id).as_str());
    } else {
        let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Error");
                e.description(format_emojis!("Not valid classpect"));
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
#[commands(information, game, set_classpect)]
pub struct Economy;