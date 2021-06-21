use serenity::model::prelude::*;
use serenity::prelude::Context;
use crate::data::DatabasePool;
use tracing::{info};

pub async fn guild_create(ctx: Context, guild: Guild, _is_new: bool) {
    let pool = ctx.data.read().await.get::<DatabasePool>().cloned().unwrap();
    let guild_id = guild.id.0 as i64;

    info!("Guild '{}' ({}) is being registered into the database.", guild.name, guild.id);

    sqlx::query("INSERT INTO \"guilds\" (guild_id, first_interaction) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(guild_id)
        .bind(guild.joined_at)
        .execute(&pool)
        .await
        .unwrap();
}