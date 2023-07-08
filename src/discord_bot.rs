use std::env;

use serenity::async_trait;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

// My UserData
struct UserData {
    channel_id: ChannelId,
}

impl TypeMapKey for UserData {
    type Value = UserData;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let data = ctx.data.read().await;
        let user_data = data.get::<UserData>().unwrap();
        let result = user_data.channel_id.send_message(&ctx.http, |m| {
            return m.content("Halo?");
        }).await;

        if let Err(why) = result {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub async fn discord_bot_main() {
    println!("main start");

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let channel_id = ChannelId(1127190786199523361);
        let user_data = UserData{channel_id};

        let mut data = client.data.write().await;
        data.insert::<UserData>(user_data);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}