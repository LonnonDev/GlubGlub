use rand::{Rng, thread_rng};
use serenity::client::Context;
use serenity::model::channel::Message;
use tokio_postgres::{Error, NoTls};

use crate::format_emojis;

const POSTGRE: &'static str = "host=192.168.1.146 user=postgres";

pub const GRIST_TYPES: (&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str) = (
    "build",
    "amber",
    "amethyst",
    "caulk",
    "chalk",
    "cobalt",
    "diamond",
    "garnet",
    "gold",
    "iodine",
    "marble",
    "mercury",
    "quartz",
    "ruby",
    "rust",
    "shale",
    "sulfur",
    "tar",
    "uranium",
    "zillium"
);

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub sprite: String,
    pub class: String,
    pub aspect: String,
    pub materials: Materials
}

#[derive(Debug, Clone)]
pub struct Materials {
    pub build: i64,
    pub amber: i64,
    pub amethyst: i64,
    pub caulk: i64,
    pub chalk: i64,
    pub cobalt: i64,
    pub diamond: i64,
    pub garnet: i64,
    pub gold: i64,
    pub iodine: i64,
    pub marble: i64,
    pub mercury: i64,
    pub quartz: i64,
    pub ruby: i64,
    pub rust: i64,
    pub shale: i64,
    pub sulfur: i64,
    pub tar: i64,
    pub uranium: i64,
    pub zillium: i64,
}

// Useful functions for Player
impl Player {
    pub fn empty() -> Self {
        return Player {
            id: 0,
            sprite: "Empty".to_string(),
            class: "Bard".to_string(),
            aspect: "Light".to_string(),
            materials: Materials::empty()
        }
    }
}

// Useful functions for Materials
impl Materials {
    pub fn empty() -> Self {
        return Materials {
            build: 0,
            amber: 0,
            amethyst: 0,
            caulk: 0,
            chalk: 0,
            cobalt: 0,
            diamond: 0,
            garnet: 0,
            gold: 0,
            iodine: 0,
            marble: 0,
            mercury: 0,
            quartz: 0,
            ruby: 0,
            rust: 0,
            shale: 0,
            sulfur: 0,
            tar: 0,
            uranium: 0,
            zillium: 0,
        }
    }
}

// Makes it so you can iterate through materials
impl IntoIterator for Materials {
    type Item = i64;
    type IntoIter = std::array::IntoIter<i64, 20>;
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([
            self.build,
            self.amber,
            self.amethyst,
            self.caulk,
            self.chalk,
            self.cobalt,
            self.diamond,
            self.garnet,
            self.gold,
            self.iodine,
            self.marble,
            self.mercury,
            self.quartz,
            self.ruby,
            self.rust,
            self.shale,
            self.sulfur,
            self.tar,
            self.uranium,
            self.zillium
        ])
    }
}

// Easily send a message
pub async fn sendmessage(message: &str, ctx: &Context, msg: &Message) {
    // Send a message or direct message the user saying there was an error
    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        if let Err(why2) = msg.author.direct_message(&ctx, |m| {
            m.content(
                format!("Hello {}, The error I got is `{}`", msg.author, why)
            )
        }).await {
            println!("{} | {}", why, why2)
        }
    }
}

// Executes a sql statement
pub async fn sqlstatement(statement: &str) -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(POSTGRE, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
    let _ = client.execute(statement, &[]).await?;
    Ok(())
}

// Checks if the user has an entry in the DB
pub async fn check_if_registered(id: u64) -> Result<(), Error> {
    // Get player
    let result = get_player(id).await;
    let player = result.unwrap_or(Player::empty());
    
    // if player.id is 0 then they don't have an entry
    // so then create an entry
    if player.id == 0 {
        let (client, connection) = tokio_postgres::connect(POSTGRE, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let _ = client.execute("INSERT INTO player 
        (\"id\") VALUES ($1);", &[&(id as i64)]).await.unwrap();
    }
    Ok(())
}

// SQLite search statement
pub async fn get_player(author_id: u64) -> Result<Player, Error> {
    let (client, connection) = tokio_postgres::connect(POSTGRE, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut player = Player::empty();

    // Create Player struct
    for row in client.query("SELECT * FROM player WHERE \"id\"=$1",&[&(author_id as i64)]).await? {
        player = Player {
            id: row.get(0),
            sprite: row.get(21),
            class: row.get(22),
            aspect: row.get(23),
            materials: Materials {
                build: row.get(1),
                amber: row.get(2),
                amethyst: row.get(3),
                caulk: row.get(4),
                chalk: row.get(5),
                cobalt: row.get(6),
                diamond: row.get(7),
                garnet: row.get(8),
                gold: row.get(9),
                iodine: row.get(10),
                marble: row.get(11),
                mercury: row.get(12),
                quartz: row.get(13),
                ruby: row.get(14),
                rust: row.get(15),
                shale: row.get(16),
                sulfur: row.get(17),
                tar: row.get(18),
                uranium: row.get(19),
                zillium: row.get(20),
            }
        }
    }

    return Ok(player)
}


// Gets exile quote
pub async fn get_exile_quote(ctx: &Context, msg: &Message) {
    // Exile quotes
    let exile_1: Vec<&str> = vec!["What are you doing", "Good job hero"];
    let exile_2: Vec<&str> = vec!["DO YOU HAVE ANY IDEA WHAT YOU ARE DOING?", "YOU ARE DOING GOOD MAGGOT!"];
    let exile_3: Vec<&str> = vec!["Good.", "Yes more."];
    let exile_4: Vec<&str> = vec!["i could do better than that", "what are you doing loser"];

    // Send embed function
    async fn send_embed(ctx: &Context, msg: &Message, embed_text: &str) {
        let randcolor: u32 = thread_rng().gen_range(0x000000..0xFFFFFF);
        if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s Exile", msg.author.name).as_str());
                e.description(format_emojis!("{}", embed_text));
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

    // Random index for exile quote
    let rand_index: u32 = thread_rng().gen_range(0..exile_1.len() as u32);

    // Send exile quote
    let author_exile = (msg.author.id.as_u64() % 4) + 1;
    if author_exile == 1 {
        send_embed(ctx, msg, exile_1[rand_index as usize]).await;
    } else if author_exile == 2 {
        send_embed(ctx, msg, exile_2[rand_index as usize]).await;
    } else if author_exile == 3 {
        send_embed(ctx, msg, exile_3[rand_index as usize]).await;
    } else if author_exile == 4 {
        send_embed(ctx, msg, exile_4[rand_index as usize]).await;
    }
}