mod token;
use crate::token::*;
mod commands;
use crate::commands::economy::ECONOMY_GROUP;
use crate::commands::owner::OWNER_GROUP;
use std::env;
use serenity::{
    async_trait,
    model::gateway::Ready,
};
use serenity::framework::standard::{
    StandardFramework,
    help_commands,
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
};
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::help;
use serenity::model::prelude::{Message, UserId};
use std::collections::HashSet;
use serenity::framework::standard::WithWhiteSpace;
use serenity::prelude::*;

pub mod useful;

struct Framework;

impl TypeMapKey for Framework {
    type Value = StandardFramework;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help]
async fn my_help(
   context: &Context,
   msg: &Message,
   args: Args,
   help_options: &'static HelpOptions,
   groups: &[&'static CommandGroup],
   owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = env::args().collect();
    let token: &str;
    let prefix: Vec<&str>;
    if args[1] == "beta" {
        println!("Beta Bot");
        token = BETA_TOKEN;
        prefix = vec!["bluh", "Bluh"]
    } else if args[1] == "stable" {
        println!("Stable Bot");
        token = STABLE_TOKEN;
        prefix = vec!["glub", "Glub"]
    } else {
        panic!("`{}` is not a valid option for `bot type`, `stable` and `beta` are valid options", args[1])
    }

    let framework = StandardFramework::new()
        .configure(|c| c.with_whitespace(WithWhiteSpace::from(true)))
        .configure(|c| c.prefixes(prefix))
        .configure(|c| c
        .owners(vec![UserId(859806257774723102)].into_iter().collect()))
        .help(&MY_HELP)
        .group(&ECONOMY_GROUP)
        .group(&OWNER_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}