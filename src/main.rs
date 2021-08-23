mod token;
use crate::token::*;
mod commands;
use crate::commands::economy::*;
use crate::commands::owner::*;
use crate::commands::general::*;
use crate::useful::sendmessage;
use std::env;
use serenity::framework::standard::DispatchError;
use serenity::framework::standard::macros::hook;
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

#[hook]
async fn dispatch_error_hook(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(rate_limit_info) => 
            sendmessage(format!("You are on cooldown for {:?}.", rate_limit_info.rate_limit).as_str(), ctx, msg)
            .await,
        DispatchError::CheckFailed(_, _) => todo!(),
        DispatchError::CommandDisabled(_) => todo!(),
        DispatchError::BlockedUser => todo!(),
        DispatchError::BlockedGuild => todo!(),
        DispatchError::BlockedChannel => todo!(),
        DispatchError::OnlyForDM => todo!(),
        DispatchError::OnlyForGuilds => todo!(),
        DispatchError::OnlyForOwners => todo!(),
        DispatchError::LackingRole => todo!(),
        DispatchError::LackingPermissions(_) => todo!(),
        _ => println!("unhandled")
    }
    
}


#[tokio::main]
async fn main() {
    let bot_type = env::args().collect::<Vec<String>>();
    let token: &str;
    let prefix: Vec<&str>;
    match bot_type.get(1).unwrap_or(&"beta".to_owned()).as_str() {
        "beta" => {
            println!("Beta Bot");
            token = BETA_TOKEN;
            prefix = vec!["bluh", "Bluh"]
        },
        "stable" => {
            println!("Stable Bot");
            token = STABLE_TOKEN;
            prefix = vec!["glub", "Glub"];
        },
        _ => {
            println!("Beta Bot");
            token = BETA_TOKEN;
            prefix = vec!["bluh", "Bluh"]
        }
    }
    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(WithWhiteSpace::from(true));
            c.prefixes(prefix);
            c.owners(vec![UserId(859806257774723102)].into_iter().collect());
            c
        })
        .help(&MY_HELP)
        .group(&ECONOMY_GROUP)
        .group(&OWNER_GROUP)
        .group(&GENERAL_GROUP)
        .bucket("basic", |b| b.delay(5).time_span(30).limit(1))
        .await
        .bucket("ring", |b| b.delay(5).time_span(60).limit(1))
        .await
        .on_dispatch_error(dispatch_error_hook);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}