use std::{env, collections::HashMap};

use reqwest::header::HeaderName;

pub async fn send_discord_message(msg: &String) {
    let hook_url = env::var("DISCORD_HOOK_URL").expect("hook url");

    // TODO iracing_client also has a request::Client. maybe we should have only one
    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("content", msg);

    client.post(hook_url)
        .json(&body)
        .send()
        .await.unwrap();
}