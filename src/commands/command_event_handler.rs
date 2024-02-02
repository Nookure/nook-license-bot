use serenity::all::{Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, GuildId, Interaction, Ready};
use serenity::async_trait;
use crate::{commands, ConfigStore};

pub struct CommandEventHandler;

#[async_trait]
impl EventHandler for CommandEventHandler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let config_lock = {
            let data_read = ctx.data.read().await;

            data_read.get::<ConfigStore>().expect("Expected CommandCounter in TypeMap.").clone()
        };

        let guild_id = GuildId::new(config_lock.bot.guild_id);

        guild_id.set_commands(&ctx.http, vec![
            commands::ping::register(),
        ]).await.expect("An error had been ocurred while registering the commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}