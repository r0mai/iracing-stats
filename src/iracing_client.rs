use std::collections::HashMap;
use serde_json;
use reqwest;

const BASEURL: &str = "https://members-ng.iracing.com";

async fn get_with_retry(client: &reqwest::Client, url: String, params: &HashMap<&str, String>) -> serde_json::Value {
    // TODO retry
    let response = client.get(url).query(&params).send().await.unwrap();
    let status = response.status();
    let text = response.text().await.unwrap();
    if status != reqwest::StatusCode::OK {
        println!("Reponse status {}", status);
        println!("Response body {}", text);
        panic!("Failed a request :(");
    }
    return serde_json::from_str(&text).unwrap();
}

async fn get_and_read(client: &reqwest::Client, url: String, params: &HashMap<&str, String>) -> serde_json::Value {
    let pointer_json = get_with_retry(client, url, params).await;
    return get_with_retry(client, String::from(pointer_json["link"].as_str().unwrap()), params).await;
}

async fn get_cust_id(client: &reqwest::Client, driver_name: &String) -> i64 {
    let body = HashMap::from([
        ("search_term", driver_name.clone())
    ]);
    let res = get_and_read(client, format!("{BASEURL}/data/lookup/drivers"), &body).await;
    let arr = res.as_array().unwrap();
    let len = arr.len();
    if len == 0 {
        panic!("Driver {driver_name} not found");
    }

    if len > 1 {
        println!("Multiple {len} matches found for {driver_name}");    
    }

    return arr[0]["cust_id"].as_i64().unwrap();
}

pub async fn sync_driver_to_db(client: &reqwest::Client, driver_name: &String) {
    let cust_id = get_cust_id(client, driver_name).await;
    println!("Cust id {cust_id}");
}

pub async fn auth(client: &reqwest::Client) {
    let user = std::env::var("IRACING_USER").unwrap();
    let token = std::env::var("IRACING_TOKEN").unwrap();

    let body = HashMap::from([
        ("email", user),
        ("password", token)
    ]);

    let response = client.post(format!("{}/auth", BASEURL)).json(&body).send().await.unwrap();
    assert!(response.status() == reqwest::StatusCode::OK);
}