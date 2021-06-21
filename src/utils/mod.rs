use crate::config::ConfigurationData;
use std::{ fs::File, io::Read };
use serenity::model::prelude::*;
use serenity::prelude::Context;
use serenity::utils::parse_username;

pub fn read_config(file: &str) -> ConfigurationData {
   let mut file = File::open(file).expect("Could not read configuration file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let asdf: ConfigurationData = serde_yaml::from_str(&contents).expect("Didnt work");
    asdf
}
pub async fn parse_user(name: &str, guild_id: GuildId, context: &Context) -> Option<UserId> {
    let guild = guild_id.to_guild_cached(&context).await.unwrap();

    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    } else if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(context, id).await {
            return Some(m.user.id);
        }
    }

    if let Some(m) = guild.member_named(name) {
        return Some(m.user.id);
    } else if let Some(m) = guild.members_starting_with(name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    } else if let Some(m) = guild.members_containing(name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    None
}


/// The user agent used for the reqwest client.
pub const REQWEST_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub const TITLE: &str = " $$$$$$\\
$$  __$$\\
$$ /  \\__| $$$$$$\\   $$$$$$\\  $$\\   $$\\
\\$$$$$$\\   \\____$$\\ $$  __$$\\ $$ |  $$ |
 \\____$$\\  $$$$$$$ |$$ |  \\__|$$ |  $$ |
$$\\   $$ |$$  __$$ |$$ |      $$ |  $$ |
\\$$$$$$  |\\$$$$$$$ |$$ |      \\$$$$$$  |
 \\______/  \\_______|\\__|       \\______/";