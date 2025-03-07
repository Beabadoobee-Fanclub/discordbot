mod commands;
mod ready;
use serenity::{
    all::{Context, EventHandler, Interaction, Message, Ready},
    async_trait,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if commands::handle_message_command(&ctx, &msg).await {
            return;
        }
    }

    async fn interaction_create(&self, ctx: Context, interation: Interaction) {
        commands::handle_interaction(&ctx, &interation).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
}
