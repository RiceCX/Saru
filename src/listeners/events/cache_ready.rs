use serenity::prelude::Context;
use serenity::model::id::{GuildId};
use std::time::Duration;
use sys_info;
use crate::data::{ConfigContainer, UptimeContainer};
use serenity::model::prelude::{ChannelId};
use tracing::{error, info};
use std::sync::Arc;
use serenity::utils::Color;
use chrono::{Utc};
use serenity::builder::Timestamp;

pub async fn cache_ready(ctx: Context, _guilds: Vec<GuildId>) {
    info!("Cache has built successfully!");


    let ctx1 = Arc::new(ctx);
    tokio::spawn(async move {
        loop {
            log_system_load(Arc::clone(&ctx1)).await;
            tokio::time::sleep(Duration::from_secs(120)).await;
        }
    });
}

async fn log_system_load(ctx: Arc<Context>) {
    let channel = ctx.data.read().await.get::<ConfigContainer>().cloned().unwrap();
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();
    let avatar = ctx.http.get_current_user().await.unwrap().avatar.unwrap();
    let avatar_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png", ctx.http.get_current_user().await.unwrap().id, avatar);
    let total_guilds = ctx.cache.guild_count().await;
    let total_users = ctx.cache.user_count().await;
    let total_shards = ctx.cache.shard_count().await;
    let uptime = ctx.data.read().await.get::<UptimeContainer>().cloned().unwrap();
    let dur = Utc::now().signed_duration_since(uptime);
    let formatted_duration = format!("{}h {}m {}s", dur.num_hours(), dur.num_minutes() % 60, dur.num_seconds() % 60);

    if let Err(why) = ChannelId(channel.bot.logging.stats_channel).edit_message(&ctx, channel.bot.logging.stats_message, |message| {
        message.embed(|e| {
          e.title("System resource Load")
              .color(Color::from_rgb(52,235,116))
              .field(
                  "Cpu Load Average",
                  format!("{:.2}%", cpu_load.one * 10.0),
                  true,
              )
              .field(
                  "Memory Usage",
                  format!("{:.2} MB / {:.2} MB", mem_use.free as f32 / 1000.0, mem_use.total as f32 / 1000.0),
                  true,
              )
              .field("Total Guilds", total_guilds, true)
              .field("Total Users", total_users, true)
              .field("Total shards", total_shards, true)
              .field("Started at", uptime.format("%a, %B %d, %Y (%r)"),true)
              .field("Uptime", formatted_duration,true)
              .field("Bot version", env!("CARGO_PKG_VERSION"), true)
              .footer(|f| {
                  f.text("Updated")
                      .icon_url(avatar_url)
              })
              .timestamp(Timestamp::from(Utc::now().to_rfc3339()))

        })
    }).await {
        error!("Error sending message: {:?}", why);
    }
}