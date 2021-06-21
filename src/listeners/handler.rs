use crate::listeners::events::{message, ready};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    model::{gateway::Ready, guild::Guild}
};
use serenity::model::id::GuildId;
use crate::listeners::events::cache_ready::cache_ready;
use crate::listeners::events::guild_create::guild_create;

pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
    async fn message(&self, ctx: Context, msg: Message) {
        message::message(ctx, msg).await;
    }
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        cache_ready(ctx, _guilds).await;
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        guild_create(ctx,guild,is_new).await;
    }
}