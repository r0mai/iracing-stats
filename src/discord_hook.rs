use std::{env, collections::HashMap};

pub async fn send_discord_update(subsession_ids: &Vec<i64>) {
    // TODO iracing_client also has a request::Client. maybe we should have only one
    let client = reqwest::Client::new();

    let hook_url = env::var("DISCORD_HOOK_URL").expect("hook url");

    let msg = format!("Synced {} subsessions", subsession_ids.len());

    println!("DISCORD_HOOK_URL = {} - msg {}", hook_url, msg);
    send_discord_message(&client, &hook_url, &msg).await;
}

pub async fn send_discord_message(client: &reqwest::Client, hook_url: &String, msg: &String) {
    let mut body = HashMap::new();
    body.insert("content", msg);

    client.post(hook_url)
        .json(&body)
        .send()
        .await.unwrap();
}