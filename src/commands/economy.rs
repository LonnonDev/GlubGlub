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
use crate::{format_items, useful::*}; 

use crate::format_emojis;

trait InVec: std::cmp::PartialEq + Sized {
    fn in_vec(self, vector: Vec<Self>) -> bool {
        vector.contains(&self)
    }
}

impl<T> InVec for T 
where
    T: std::cmp::PartialEq
{}

trait ConvertCaseToSnake {
    fn to_snakecase(&self) -> String;
}

impl ConvertCaseToSnake for String {
    fn to_snakecase(&self) -> String {
        let part1 = &self.to_uppercase()[0..1];
        let part2 = &self.to_lowercase()[1..self.len()];
        return format!("{}{}", part1, part2);
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

trait FormatVec {
    fn format_vec(&self) -> String;
}

impl<T> FormatVec for Vec<T> where T: std::fmt::Display {
    fn format_vec(&self) -> String {
        let mut return_string = "".to_owned();
        for x in self {
            return_string = format!("{}\n{}", return_string, x);
        }
        println!("{:?}", return_string);
        if return_string.replace("\n", "") == "" {
            return "Empty".to_owned()
        } else {
            return return_string
        }
    }
}

#[command]
async fn use_sylladex(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let action = args.single::<String>().unwrap_or("nothing".to_owned()).to_lowercase();
    let player = get_player(*msg.author.id.as_u64()).await?;
    match player.sylladex_type.as_str() {
        "stack" => match action.as_str() {
            "push" => (),
            "pop" => (),
            "push_storage" => (),
            _ => sendmessage("Invalid Action\nValid actions are: `push`, `pop`, `push_storage`", ctx, msg).await,
        },
        _ => (),
    }

    Ok(())
}

#[command]
#[aliases("info")]
async fn information(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get author id
    let author_id = args.single::<UserId>().unwrap_or(msg.author.id);
    let author = msg.guild_id.unwrap().member(ctx, author_id).await.unwrap().user;

    // Registers the user if they don't exist
    let _ = check_if_registered(*author_id.as_u64()).await?;

    // Get the players grist
    let player = get_player(*author_id.as_u64()).await.unwrap();

    // Put all of the grist the user has in a string
    let mut info_message = String::new();
    let mut y = 0;
    for x in player.clone().materials {
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
    send_embed(ctx, msg, |e| {
        e.title(format!("{}'s Player info", author.name).as_str());
        e.color(randcolor);
        e.author(|a| {
            a.icon_url(author.avatar_url().unwrap());
            a.name(author.name.as_str());
            a
        });
        e.field("Classpect", format_emojis!("{} of {} :{}:", player.class, player.aspect, player.aspect.to_lowercase()), false);
        e.field("Grist", format_emojis!("{}", info_message), false);
        e.field("Inventory", format_items!("{}", player.inventory.format_vec()), false);
        e.field("Storage", format_items!("{}", player.storage.format_vec()), false);
        e.field("Sylladex", format!("{}", player.sylladex_type.to_string().to_snakecase()), false);
        e
    }).await;

    Ok(())
}

#[command]
#[bucket("basic")]
async fn game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get author id
    let author_id = args.single::<UserId>().unwrap_or(msg.author.id);
    let author = msg.guild_id.unwrap().member(ctx, author_id).await.unwrap().user;

    // Registers the user if they don't exist
    let _ = check_if_registered(*author_id.as_u64()).await.unwrap();

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
    let result = get_player(*author_id.as_u64()).await;
    let player = result.unwrap();

    // Get the new grist value, and update the player
    let newvalue = random_grist + player.materials.build;
    let _ = sqlstatement(format!("UPDATE player SET build={} WHERE \"id\"={}", newvalue, author.id.as_u64()).as_str()).await?;

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



// Sets a user classpect
#[command]
async fn set_classpect(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    check_if_registered(*msg.author.id.as_u64()).await?;

    let classes = vec!["Bard", "Prince", "Heir", "Page", "Seer", "Maid", "Sylph", "Knight", "Knight", "Witch", "Mage"];
    let aspects = vec!["Space", "Time", "Light", "Void", "Heart", "Mind", "Hope", "Rage", "Life", "Doom", "Breath", "Blood"];

    let author_id = *msg.author.id.as_u64();
    let classpect = vec![args.single::<String>().unwrap(), args.single::<String>().unwrap(), args.single::<String>().unwrap()];

    // Make sure it's a valid classpect
    if classpect[0].to_snakecase().as_str().in_vec(classes.clone()) && classpect[1].to_lowercase() == "of" && classpect[2].to_snakecase().as_str().in_vec(aspects.clone()) {
        let _ = sqlstatement(format!("UPDATE player SET \"class\"='{}', aspect='{}' WHERE \"id\"={}", classpect[0].to_snakecase(), classpect[2].to_snakecase(), author_id).as_str()).await?;
        sendmessage("Set your classpect successfully", ctx, msg).await;
    } else {
        let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, move |m| {
            m.embed(|e| {
                e.title("Error");
                e.description(format_emojis!("Not valid classpect"));
                e.description("Please provide classpects in the format of [CLASS] of [ASPECT]");
                e.field("Classes", format!("{:?}", classes), true);
                e.field("Aspects", format!("{:?}", aspects), true);
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
#[commands(information, game, set_classpect, use_sylladex)]
pub struct Economy;