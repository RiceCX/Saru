use serenity::prelude::*;
use serenity::model::prelude::Message;
use crate::data::DatabasePool;
use chrono::{ Utc };

pub async fn message(ctx: Context, message: Message) {
    let pool = ctx.data.read().await.get::<DatabasePool>().cloned().unwrap();

    let user_id = message.author.id.0 as i64;

    if !message.author.bot {
        sqlx::query("INSERT INTO \"users\" (user_id, first_interaction) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(user_id)
            .bind(Utc::now())
            .execute(&pool)
            .await
            .unwrap();
    }
}