use std::{error::Error, sync::Arc};
use futures::stream::StreamExt;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::{Cluster, ShardScheme}, Event};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;
use discord_commands::{
    ping,
    help
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = "token";

    // This is the default scheme. It will automatically create as many
    // shards as is suggested by Discord.
    let scheme = ShardScheme::Auto;

    // Use intents to only receive guild message events.
    let (cluster, mut events) = Cluster::builder(&token.to_string(), Intents::GUILD_MESSAGES)
        .shard_scheme(scheme)
        .build()
        .await?;
    let cluster = Arc::new(cluster);

    // Start up the cluster.
    let cluster_spawn = Arc::clone(&cluster);

    // Start all shards in the cluster in the background.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token.to_string()));

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Process each event as they come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            ping::ping(http, msg).await;
        }
        Event::MessageCreate(msg) if msg.content == "!help" => {
            help::help(http, msg).await;
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
