use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_embed_builder::{
    EmbedBuilder,
    EmbedFieldBuilder
};
use twilight_model::channel::embed::Embed;
use discord_utils::colors::Colors;
use twilight_model::guild::Ban;
use twilight_model::user::User;

pub async fn help(http: Arc<HttpClient>, msg: Box<MessageCreate>, user: User) {
    let embed = EmbedBuilder::new()
        .description("Help embed.")
        .field(EmbedFieldBuilder::new("!ping", "Send `Pong!`."))
        .field(EmbedFieldBuilder::new("!help", "Send this embed."))
        .color(Colors::Green.to_color())
        .build()
        .unwrap();

    let embeds: &[Embed] = &vec![embed];

    http.create_message(msg.channel_id)
        .embeds(embeds)
        .unwrap()
        .exec()
        .await
        .unwrap();

    http.create_ban(msg.guild_id, msg.member.)
}