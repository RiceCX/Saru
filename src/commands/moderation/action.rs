use serenity::{client::Context, framework::standard::{macros::command, CommandResult}, model::{
    prelude::Message
}, Error};
use serenity::framework::standard::Args;
use crate::utils::parse_user;
use serenity::builder::{CreateButton, CreateActionRow};
use serenity::model::prelude::{ReactionType, ButtonStyle, InteractionResponseType, Member, Interaction, User};
use serenity::model::prelude::Target::Emoji;
use std::time::Duration;
use tracing::{info, trace, instrument};
use serenity::model::interactions::InteractionData;
use serenity::futures::StreamExt;
use std::sync::Arc;
use crate::listeners::events::message::message;
use serenity::model::id::RoleId;
use crate::data::DatabasePool;
use sqlx::types::chrono::Utc;

#[command]
#[description = "Shows various information about the current guild."]
#[min_args(1)]
async fn action(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.is_private() {
        msg.reply(context, "You can't kick anyone in private messages!").await?;
        return Ok(());
    }
    let mention = args.single_quoted::<String>()?;
    let guild = msg.guild_id.unwrap();
    let user = parse_user(&mention, guild, context).await.unwrap();
    let mut member = msg.guild(context).await.unwrap().member(context, user).await.unwrap();
    let action_msg = msg.channel_id.send_message(&context.http, |m| {
        m.content(format!("What would you like to do to `{}` (`{}`)?", member.user.tag(), member.user.id))
        .components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|cb| {
                    cb.style(ButtonStyle::Primary)
                        .label("Mute")
                        .custom_id("btn-mute")
                }).create_button(|cb| {
                    cb.style(ButtonStyle::Secondary)
                        .label("Kick")
                        .custom_id("btn-kick")
                }).create_button(|cb| {
                        cb.style(ButtonStyle::Danger)
                            .label("Ban")
                            .custom_id("btn-ban")
                }).create_button(|cb| {
                    cb.style(ButtonStyle::Success)
                        .label("View Logs")
                        .custom_id("btn-logs")
                })
            })
        })
    }).await?;
    if let Some(answer) = action_msg.await_component_interaction(&context).timeout(Duration::from_secs(60)).author_id(msg.author.id).await {
        match answer.data.as_ref().unwrap() {
            InteractionData::MessageComponent(b) => {
                match b.custom_id.as_str() {
                    "btn-mute" => {
                        followup_mute(&context, &answer, &msg.author, &mut member, &action_msg).await;
                    },
                    "btn-kick" => {
                        answer.create_interaction_response(&context.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|f| {
                                    f.content(format!("User `{}` (`{}`) has been kicked.", member.user.tag(), member.user.id))
                                })
                        }).await;
                    },
                    "btn-ban" => {
                        answer.create_interaction_response(&context.http, |f| {
                            f.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|f| {
                                    f.content(format!("User `{}` (`{}`) has been banned.", member.user.tag(), member.user.id))
                                })
                        }).await;
                    },
                    "btn-logs" => {
                        followup_logs(&context, &answer, &msg.author, &member).await;
                    }
                    _ => info!("they entered some randomb ullshit")
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Debug)]
enum ACTION {
    MUTE,
    KICK,
    BAN,
    VIEW_LOGS
}
async fn followup_mute(ctx: &Context, interaction: &Interaction, issuer: &User, target: &mut Member, msg: &Message) {
    let mut followup_msg = "An error occurred.".to_string();
    info!("Trying to mute user {} by user {}", target.user.id, issuer.id);
    interaction.create_interaction_response(&ctx.http, |f| {
        f.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|d| {
                d.content("How long would you like to mute for?");
                d.components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|b| {
                            b.custom_id("forever")
                                .style(ButtonStyle::Danger)
                                .label("Forever")
                        }).create_button(|b| {
                            b.custom_id("custom")
                                .style(ButtonStyle::Primary)
                                .label("Custom")
                        })
                    })
                })
            })
    }).await.expect("ERRORS");

    if let Some(a) = msg.await_component_interaction(&ctx).timeout(Duration::from_secs(60)).author_id(issuer.id).await {
        match a.data.as_ref().unwrap() {
            InteractionData::MessageComponent(b) => {
                match b.custom_id.as_str() {
                    "forever" => {
                        info!("They chosed forever.");
                        a.create_interaction_response(&ctx.http, |f| {
                            f.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|d| {
                                   d.content("Would you like to provide a reason?");
                                   d.components(|c| {
                                       c.create_action_row(|ar| {
                                           ar.create_button(|b| {
                                               b.custom_id("yes")
                                                   .style(ButtonStyle::Primary)
                                                   .label("Yes")
                                           }).create_button(|b| {
                                               b.custom_id("no")
                                                   .style(ButtonStyle::Primary)
                                                   .label("No")
                                           })
                                       })
                                   })
                                   })
                        }).await.expect("what the fuck");

                        info!("created reason interaction.");
                        if let Some(a) = msg.await_component_interaction(&ctx).timeout(Duration::from_secs(60)).author_id(issuer.id).await {
                            match a.data.as_ref().unwrap() {
                                InteractionData::MessageComponent(b) => {
                                    match b.custom_id.as_str() {
                                        "yes" => {
                                            info!("They wanted to create a reason");
                                            a.create_interaction_response(&ctx.http,  |f| {
                                                f.kind(InteractionResponseType::UpdateMessage)
                                                    .interaction_response_data(|rd| {
                                                        rd.content("Type your reason.")
                                                            .components(|c| {
                                                            let clear_vec = vec![];
                                                            c.set_action_rows(clear_vec)
                                                        })
                                                    })

                                            }).await;
                                            if let Some(d) = interaction.channel_id.unwrap().await_reply(&ctx).timeout(Duration::from_secs(15)).author_id(issuer.id).await {
                                                followup_msg = mute_user(&ctx, &issuer, target, &d.content).await;
                                                create_response(&ctx, &a, &followup_msg).await;
                                                return;
                                            }
                                        },
                                        "no" => {
                                            followup_msg = mute_user(&ctx, &issuer, target, "None").await;
                                            create_response_interaction(&ctx, &a, &followup_msg).await;
                                            return;
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        return
                    },
                    "custom" => {
                        info!("Hi");
                    },
                    _ => {
                        info!("No shit provided.");
                    }
                }
            }
            _ => {
                info!("error!")
            }
        }
    }
}

async fn mute_user(ctx: &Context, issuer: &User, target: &mut Member, reason: &str) -> String {
    let mut followup_msg= "There was an error.".parse().unwrap();
    if let err = target.add_role(&ctx.http, 704641046465740851).await {
        match err {
            Ok(_) => {
                followup_msg = format!("User `{}` (`{}`) has been muted.", target.user.tag(), target.user.id);
                log_action(ctx, ACTION::MUTE, issuer.id.0 as i64, target.user.id.0 as i64, reason, target.guild_id.0 as i64).await;
            }
            Err(e) => {
                followup_msg = format!("User `{}` (`{}`) could not be muted! Might be a lack of permissions or this role doesn't exist.\n**{}**", target.user.tag(), target.user.id, e.to_string());
            }
        }
    }
    followup_msg
}

async fn create_response(ctx: &Context, interaction: &Interaction, followup_msg: &str) {
    info!("Creating final response!");
    interaction.create_followup_message(&ctx.http, |f| {
                f.content(followup_msg)
    }).await.expect("what the fuck error?");
}
async fn create_response_interaction(ctx: &Context, interaction: &Interaction, followup_msg: &str) {
    info!("Creating final response!");
    interaction.create_interaction_response(&ctx.http, |f| {
        f.kind(InteractionResponseType::UpdateMessage)
            .interaction_response_data(|rd| {
                rd.content(followup_msg)
                    .components(|c| {
                        let clear_vec = vec![];
                        c.set_action_rows(clear_vec)
                    })
            })
    }).await.expect("what the fuck error?");
}
/*
async fn followup_kick(ctx: &Context, interaction: &Interaction,issuer: &User, target: &Member) {
    info!("Trying to mute user {} by user {}", target.user.id, issuer.id);
    interaction.create_interaction_response(&ctx.http, |f| {
        f.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|f| {
                f.content(format!("User `{}` (`{}`) has been niggered.", target.user.tag(), target.user.id))
            })
    }).await;
}

async fn followup_ban(ctx: &Context, interaction: &Interaction,issuer: &User, target: &Member) {
    info!("Trying to mute user {} by user {}", target.user.id, issuer.id);
    interaction.create_interaction_response(&ctx.http, |f| {
        f.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|f| {
                f.content(format!("User `{}` (`{}`) has been niggered.", target.user.tag(), target.user.id))
            })
    }).await;
}
*/

async fn followup_logs(ctx: &Context, interaction: &Interaction,issuer: &User, target: &Member) {
    log_action(&ctx, ACTION::VIEW_LOGS, issuer.id.0 as i64, target.user.id.0 as i64, "None", target.guild_id.0 as i64).await;
    let pool = ctx.data.read().await.get::<DatabasePool>().cloned().unwrap();
    interaction.create_interaction_response(&ctx.http, |f| {
        f.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|f| {
                f.content(format!("User `{}` (`{}`) has been niggered.", target.user.tag(), target.user.id))
            })
    }).await;
}


async fn log_action(ctx: &Context, action: ACTION, issuer: i64, target: i64, reason: &str, guild: i64) {
    trace!("User {} is {:?} {} with reason: {}", issuer, action,target,reason);
    let pool = ctx.data.read().await.get::<DatabasePool>().cloned().unwrap();
    match action {
        ACTION::MUTE => {
            sqlx::query("INSERT INTO \"actions\" (user_id, guild_id, type, timestamp, issuer_id, reason) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING")
                .bind(target)
                .bind(guild)
                .bind("MUTE")
                .bind(Utc::now())
                .bind(issuer)
                .bind(reason)
                .execute(&pool)
                .await
                .unwrap();
        }
        ACTION::KICK => {
            sqlx::query("INSERT INTO \"actions\" (user_id, guild_id, type, timestamp, issuer_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind(target)
                .bind(guild)
                .bind("KICK")
                .bind(Utc::now())
                .bind(issuer)
                .bind(reason)
                .execute(&pool)
                .await
                .unwrap();
        }
        ACTION::BAN => {
            sqlx::query("INSERT INTO \"actions\" (user_id, guild_id, type, timestamp, issuer_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind(target)
                .bind(guild)
                .bind("BAN")
                .bind(Utc::now())
                .bind(issuer)
                .bind(reason)
                .execute(&pool)
                .await
                .unwrap();
        }
        ACTION::VIEW_LOGS => {
            sqlx::query("INSERT INTO \"actions\" (user_id, guild_id, type, timestamp, issuer_id) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind(target)
                .bind(guild)
                .bind("VIEW_LOGS")
                .bind(Utc::now())
                .bind(issuer)
                .bind(reason)
                .execute(&pool)
                .await
                .unwrap();
        }
    }
}