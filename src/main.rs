use std::{collections::HashSet, error::Error, sync::Arc};

use chrono::Utc;
use reqwest::{Client, redirect::Policy};
use serenity::{
    client::{bridge::gateway::GatewayIntents, ClientBuilder},
    framework::{standard::macros::group, StandardFramework},
    http::Http
};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::data::*;
use crate::listeners::handler::Handler;
use crate::listeners::hooks::{after, before, dispatch_error, prefix_only};
use crate::utils::{read_config, REQWEST_USER_AGENT, TITLE};

mod config;
mod utils;
mod data;
mod listeners;
mod commands;

use commands::{
    info::{test::*}
};

#[group("Info")]
#[description = "Informational commands that provide useful information."]
#[commands(test)]
struct Info;

#[tokio::main(worker_threads = 16)]
#[instrument]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>>{
    print!("\x1B[2J\x1B[1;1H");
    println!("{}\n\n", TITLE);
    let config = read_config("configuration.yml");
    let logging = config.bot.logging.enabled;

    if logging {
        LogTracer::init()?;

        let base_level = config.bot.logging.level.as_str();

        let level = match base_level {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::TRACE
        };

        let subscriber = FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(level)
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("Could not set global subscriber");

        info!("Tracing started with logging level set to {}.", level);
    }

    let appid = config.bot.application.id;
    let token = config.bot.application.token;
    let prefix = config.bot.prefix.as_str();

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => {
            error!("Unable to retrieve application info: {:?}", why);
            return Ok(());
        }
    };

    let framework = StandardFramework::new()
        .configure(|configuration| {
            configuration
                .on_mention(Some(bot_id))
                .prefix(prefix)
                .ignore_webhooks(true)
                .ignore_bots(true)
                .no_dm_prefix(true)
                .with_whitespace(true)
                .owners(owners)
                .case_insensitivity(true)
        })
        .before(before)
        .after(after)
        .prefix_only(prefix_only)
        .on_dispatch_error(dispatch_error)
        .group(&INFO_GROUP);

    let mut client = ClientBuilder::new(&token)
        .event_handler(Handler)
        .application_id(appid)
        .intents(GatewayIntents::all())
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;

        let http_client = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;

        let url = config.database.url;
        let pool = PgPoolOptions::new().max_connections(20).connect(&url).await?;

        data.insert::<ConfigContainer>(read_config("configuration.yml"));
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestContainer>(http_client);
        data.insert::<DatabasePool>(pool);
        data.insert::<UptimeContainer>(Utc::now());

    }
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        info!("Shutting down!");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
