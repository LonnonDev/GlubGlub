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

const DATABASE_PATH: &str = "../Database.db";

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
        IntoIterator::into_iter([
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

//* Checks if the user has an entry in the DB
pub fn check_if_registered(msg: &Message) -> Result<()> {
    let result = search_statement(format!("SELECT * FROM player WHERE id={}", msg.author.id).as_str());
    let player = result.unwrap();
    println!("{}", player.id);
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
        .replace(":build:", "<:Build:862808331004542987>")
        .replace(":amber:", "<:Amber:862808330875699223>")
        .replace(":amethyst:", "<:Amethyst:862808331155144704>")
        .replace(":caulk:", "<:Caulk:862808330937434163>")
        .replace(":chalk:", "<:Chalk:862808330833494037>")
        .replace(":cobalt:", "<:Cobalt:862808330934419496>")
        .replace(":diamond:", "<:Diamond:862808330501357599>")
        .replace(":garnet:", "<:Garnet:862808330892345354>")
        .replace(":gold:", "<:Gold:862808330846208031>")
        .replace(":iodine:", "<:Iodine:862808330904010792>")
        .replace(":marble:", "<:Marble:862808330846208032>")
        .replace(":mercury:", "<:Mercury:862808330896146452>")
        .replace(":quartz:", "<:Quartz:862808330836770866>")
        .replace(":ruby:", "<:Ruby:862808330464264225>")
        .replace(":rust:", "<:Rust:862808330556932097>")
        .replace(":shale:", "<:Shale:862808330874912808>")
        .replace(":sulfur:", "<:Sulfur:862808330815668265>")
        .replace(":tar:", "<:Tar:862808330833494036>")
        .replace(":uranium:", "<:Uranium:862808330501357598>")
        .replace(":zillium:", "<:Zillion:862808330644095008>");
    return new_text
}