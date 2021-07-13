mod token;
use crate::token::*;
mod economy;
use crate::economy::ECONOMY_GROUP;
use std::env;
use serenity::{
    async_trait,
    model::gateway::Ready,
};
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let args: Vec<String> = env::args().collect();
    let token: &str;
    let prefix: Vec<&str>;
    if args[1] == "beta" {
        token = BETA_TOKEN;
        prefix = vec!["bluh", "Bluh"]
    } else {
        token = STABLE_TOKEN;
        prefix = vec!["glub", "Glub"]
    }

    let framework = StandardFramework::new()
    .configure(|c| c.prefixes(prefix))
    // The `#[group]` (and similarly, `#[command]`) macro generates static instances
    // containing any options you gave it. For instance, the group `name` and its `commands`.
    // Their identifiers, names you can use to refer to these instances in code, are an
    // all-uppercased version of the `name` with a `_GROUP` suffix appended at the end.
    .group(&ECONOMY_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}