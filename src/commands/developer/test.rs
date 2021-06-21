use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        prelude::Message
    }
};

#[command]
#[description = "Shows various information about the current guild."]
async fn test(context: &Context, message: &Message) -> CommandResult {
    message.reply(&context.http, "shut up").await.expect("Could not send message");

    Ok(())
}
