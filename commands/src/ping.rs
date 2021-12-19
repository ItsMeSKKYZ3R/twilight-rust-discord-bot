use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

/// The Ping command of our Rust Discord bot.
///
/// Use:
/// ```rs
/// use commands::ping;
///
/// fn main() {
///     ping::ping().await;
/// }
/// ```
pub async fn ping(http: Arc<HttpClient>, msg: Box<MessageCreate>) {
    http.create_message(msg.channel_id)
        .content("Pong!")
        .unwrap()
        .exec()
        .await
        .unwrap();
}