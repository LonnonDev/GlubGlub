use serenity::client::Context;
use serenity::model::channel::Message;
use rusqlite::{Connection, Result, params};

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

const DATABASE_PATH: &str = "./database.db";

pub struct Player {
    pub id: i64,
    pub materials: Materials
}

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

//* Useful functions for Player
impl Player {
    pub fn empty() -> Self {
        return Player {
            id: 0,
            materials: Materials::empty()
        }
    }
}

//* Useful functions for Materials
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

//* Makes it so you can iterate through materials
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

//* Easily send a message
pub async fn sendmessage(message: &str, ctx: &Context, msg: &Message) {
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

//* Executes a sql statement
pub fn sqlstatement(statement: &str) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;
    let addplayer = conn.execute(statement, params![]);
    if let Err(err) = conn.close() {println!("{}", err.1);}
    if let Err(err) = addplayer {println!("{}", err)}
    Ok(())
}

//* Checks if the user has an entry in the DB
pub fn check_if_registered(msg: &Message) -> Result<()> {
    let result = search_statement(format!("SELECT * FROM player WHERE id={}", msg.author.id).as_str());
    let player = result.unwrap_or(Player { id: 0, materials: Materials::empty() });
    //# if `player.id` is 0 then they don't have an entry
    if player.id == 0 {
        let conn = Connection::open(DATABASE_PATH)?;
        let addplayer = conn.execute("INSERT INTO player (id, build, amber, amethyst, caulk, chalk, cobalt, diamond, garnet, gold, iodine, marble, mercury, quartz, ruby, rust, shale, sulfur, tar, uranium, zillium) VALUES (?, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)", params![msg.author.id.as_u64()]);
        if let Err(err) = conn.close() {println!("{}", err.1);}
        if let Err(err) = addplayer {println!("{}", err)}
    }

    Ok(())
}

//* SQLite search statement
pub fn search_statement(statement: &str) -> Result<Player> {
    let conn = Connection::open(DATABASE_PATH)?;
    let mut stmt = conn.prepare(
        statement,
    )?;
    
    //# Create `Player` struct
    let player_iter = stmt.query_map([], |row| {
        Ok(Player {
            id: row.get(0)?,
            materials: Materials {
                build: row.get(1)?,
                amber: row.get(2)?,
                amethyst: row.get(3)?,
                caulk: row.get(4)?,
                chalk: row.get(5)?,
                cobalt: row.get(6)?,
                diamond: row.get(7)?,
                garnet: row.get(8)?,
                gold: row.get(9)?,
                iodine: row.get(10)?,
                marble: row.get(11)?,
                mercury: row.get(12)?,
                quartz: row.get(13)?,
                ruby: row.get(14)?,
                rust: row.get(15)?,
                shale: row.get(16)?,
                sulfur: row.get(17)?,
                tar: row.get(18)?,
                uranium: row.get(19)?,
                zillium: row.get(20)?,
            }
        })
    })?;
    let mut return_value = Player::empty();
    for player in player_iter {
        return_value = player.unwrap();
    }
    return Ok(return_value)
}

//* Replaces :emojis: with actual emojis
pub fn format_emojis(text: String) -> String {
    let new_text: String = text
        .replace(":build:", "<:build:878027836319989790>")
        .replace(":amber:", "<:amber:878027835531468801>")
        .replace(":amethyst:", "<:amethyst:878027835959296010>")
        .replace(":artifact:", "<:artifact:878027835913142292>")
        .replace(":caulk:", "<:caulk:878027835959296011>")
        .replace(":chalk:", "<:chalk:878027836261294091>")
        .replace(":cobalt:", "<:cobalt:878027836072542238>")
        .replace(":diamond:", "<:diamond:878027836093526036>")
        .replace(":garnet:", "<:garnet:878027836093521940>")
        .replace(":gold:", "<:gold:878027835808301108>")
        .replace(":iodine:", "<:iodine:878027836273864774>")
        .replace(":marble:", "<:marble:878027836093521941>")
        .replace(":mercury:", "<:mercury:878027836093521933>")
        .replace(":quartz:", "<:quartz:878027835929931907>")
        .replace(":ruby:", "<:ruby:878027836248690788>")
        .replace(":rust:", "<:rust:878027836210941953>")
        .replace(":shale:", "<:shale:878027835808301109>")
        .replace(":sulfur:", "<:sulfur:878027836278063174>")
        .replace(":tar:", "<:tar:878027836504559626>")
        .replace(":uranium:", "<:uranium:878027836269674537>")
        .replace(":zillion:", "<:zillion:878027836093521942>");
    return new_text
}