use std::collections::HashMap;
use serde_json;
use reqwest::{self, Client};

const BASEURL: &str = "https://members-ng.iracing.com";

async fn get_with_retry(client: &Client, url: String, params: &HashMap<&str, String>) -> serde_json::Value {
    for _ in 0..5 {
        let response = client.get(&url).query(&params).send().await.unwrap();
        let status = response.status();
        let text = response.text().await.unwrap();
        status.is_server_error();
        if status.is_success() {
            return serde_json::from_str(&text).unwrap();
        }

        if status.is_server_error() {
            println!("Request to {url} failed with {status}. Retrying...");
            continue;
        }
        println!("Reponse status {}", status);
        println!("Response body {}", text);
        panic!("Failed a request :(");
    }
    panic!("Failed after several retries :(");
}

async fn get_and_read(client: &Client, suffix: &str, params: &HashMap<&str, String>) -> serde_json::Value {
    let pointer_json = get_with_retry(client, format!("{BASEURL}{suffix}"), params).await;
    return get_with_retry(client, String::from(pointer_json["link"].as_str().unwrap()), params).await;
}

async fn get_and_read_chunked(client: &Client, suffix: &str, params: &HashMap<&str, String>) -> serde_json::Value {
    let pointer_json = get_with_retry(client, format!("{BASEURL}{suffix}"), params).await;
    let chunk_info = &pointer_json["data"]["chunk_info"];
    let base_url_res= &chunk_info["base_download_url"].as_str();

    let mut result_array = serde_json::Value::Array([].to_vec());
    if base_url_res.is_none() {
        return result_array;
    }

    let base_url = base_url_res.unwrap();

    for file in chunk_info["chunk_file_names"].as_array().unwrap() {
        let suffix = file.as_str().unwrap();
        let mut partial_result = get_with_retry(client, format!("{base_url}{suffix}"), &HashMap::new()).await;
        result_array.as_array_mut().unwrap().append(partial_result.as_array_mut().unwrap());
    }

    return result_array;
}

async fn get_member_since_year(client: &Client, cust_id: i64) -> i32 {
    let params = HashMap::from([
        ("cust_ids", cust_id.to_string())
    ]);

    let res = get_and_read(client, "/data/member/get", &params).await;

    return res["members"][0]["member_since"].as_str().unwrap()[0..4].parse::<i32>().unwrap();
}

async fn search_series(client: &Client, cust_id: Option<i64>, year: i32, quarter: i32, week: Option<i32>) -> serde_json::Value {
    let mut params = HashMap::from([
        ("season_year", year.to_string()),
        ("season_quarter", quarter.to_string()),
    ]);

    if let Some(cust_id) = cust_id {
        params.insert("cust_id", cust_id.to_string());
    }
    if let Some(week) = week {
        params.insert("race_week_num", week.to_string());
    }
    return get_and_read_chunked(client, "/data/results/search_series", &params).await;
}

async fn find_subsessions_for_driver(client: &Client, cust_id: i64) -> Vec<i64> {
    let member_since_year = get_member_since_year(client, cust_id).await;

    let mut series = serde_json::Value::Array([].to_vec());
    for year in member_since_year..2023+1 {
        for quarter in 1..4+1 {
            println!("Query {year}s{quarter}");
            let mut series_q = search_series(client, Some(cust_id), year, quarter, None).await;
            series.as_array_mut().unwrap().append(series_q.as_array_mut().unwrap());
        }
    }

    return series.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
}

async fn find_subsessions_for_season(client: &Client, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
    let series = search_series(client, None, year, quarter, week).await;
    return series.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
}

fn filter_non_cached(subsessions: Vec<i64>) -> Vec<i64> {
    let len1 = subsessions.len();
    let res: Vec<_> = subsessions.into_iter().filter(|ses| !crate::db::is_session_cached(*ses)).collect();
    let len2 = res.len();
    println!("Non-cached sessions {len2}/{len1}");
    return res;
}

async fn find_non_cached_subsessions_for_driver(client: &Client, cust_id: i64) -> Vec<i64> {
    return filter_non_cached(find_subsessions_for_driver(client, cust_id).await);
}

async fn find_non_cached_subsessions_for_season(client: &Client, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
    return filter_non_cached(find_subsessions_for_season(client, year, quarter, week).await);
}

async fn get_cust_id(client: &Client, driver_name: &String) -> i64 {
    let res = get_and_read(client, "/data/lookup/drivers", &HashMap::from([
        ("search_term", driver_name.clone())
    ])).await;
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

async fn sync_subsession(client: &Client, subsession_id: i64, prefix: &str) {
    if crate::db::is_session_cached(subsession_id) {
        return;
    }

    println!("{prefix}Syncing session {subsession_id}");

    let res = get_and_read(client, "/data/results/get", &HashMap::from([
        ("subsession_id", subsession_id.to_string())
    ])).await;

    crate::db::write_cached_session_json(subsession_id, &res);
}

async fn sync_subsessions(client: &Client, subsession_ids: &Vec<i64>) {
    let len = subsession_ids.len();
    println!("Syncing {len} subsessions");

    for (i, subsession_id) in subsession_ids.into_iter().enumerate() {
        sync_subsession(client, *subsession_id, format!("{i}/{len} ").as_str()).await;
    }
}

fn add_subsessions_to_db(subsession_ids: &Vec<i64>) {
    let mut con = rusqlite::Connection::open(crate::db::SQLITE_DB_FILE).unwrap();
    let mut tx = con.transaction().unwrap();
    {
        let mut ctx = crate::db::create_db_context(&mut tx);

        for subsession_id in subsession_ids {
            crate::db::add_session_to_db_from_cache(&mut ctx, *subsession_id);
        }
    }

    tx.commit().unwrap();
}

pub async fn sync_drivers_to_db(client: &Client, driver_names: &Vec<String>) {
    let mut subsession_ids = Vec::new();

    for driver_name in driver_names {
        let cust_id = get_cust_id(client, driver_name).await;
        println!("Cust id {cust_id}");

        subsession_ids.append(&mut find_non_cached_subsessions_for_driver(client, cust_id).await);
    }
    sync_subsessions(client, &subsession_ids).await;
    add_subsessions_to_db(&subsession_ids);
}

pub async fn sync_season_to_db(client: &Client, year: i32, quarter: i32, week: Option<i32>) {
    let subsession_ids = find_non_cached_subsessions_for_season(client, year, quarter, week).await;
    sync_subsessions(client, &subsession_ids).await;
    add_subsessions_to_db(&subsession_ids);
}

pub async fn auth(client: &Client) {
    let user = std::env::var("IRACING_USER").unwrap();
    let token = std::env::var("IRACING_TOKEN").unwrap();

    let body = HashMap::from([
        ("email", user),
        ("password", token)
    ]);

    let response = client.post(format!("{}/auth", BASEURL)).json(&body).send().await.unwrap();
    assert!(response.status() == reqwest::StatusCode::OK);
}