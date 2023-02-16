use std::{
    collections::HashMap,
    collections::HashSet,
    sync::atomic::AtomicI64,
    sync::atomic::Ordering
};
use serde_json;
use reqwest::{self, Client, header::HeaderValue};
use std::time::Instant;

const BASEURL: &str = "https://members-ng.iracing.com";

pub struct IRacingClient {
    pub client: Client,
    pub rate_limit_limit: AtomicI64,
    pub rate_limit_remaining: AtomicI64,
    pub rate_limit_reset: AtomicI64,
}

impl IRacingClient {
    pub fn new() -> IRacingClient {
        let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
        return IRacingClient {
            client,
            rate_limit_limit: AtomicI64::new(1),
            rate_limit_remaining: AtomicI64::new(1),
            rate_limit_reset: AtomicI64::new(0)
        };
    }

    fn header_value_to_i64(v: &HeaderValue) -> i64 {
        return v.to_str().unwrap().parse::<i64>().unwrap();
    }

    async fn get_with_retry(&self, url: String, params: &HashMap<&str, String>) -> serde_json::Value {
        for _ in 0..10 {
            let response = self.client.get(&url).query(&params).send().await.unwrap();
            let status = response.status();
            let headers = response.headers();
            let rl_limit = headers.get("x-ratelimit-limit");
            let rl_remaining = headers.get("x-ratelimit-remaining");
            let rl_reset = headers.get("x-ratelimit-reset");

            if let Some(x) = rl_limit { self.rate_limit_limit.store(Self::header_value_to_i64(x), Ordering::Relaxed); }
            if let Some(x) = rl_remaining { self.rate_limit_remaining.store(Self::header_value_to_i64(x), Ordering::Relaxed); }
            if let Some(x) = rl_reset { self.rate_limit_reset.store(Self::header_value_to_i64(x), Ordering::Relaxed); }

            let text = response.text().await.unwrap();
            status.is_server_error();
            if status.is_success() {
                return serde_json::from_str(&text).unwrap();
            }

            if status.is_server_error() {
                println!("Request to {url} failed with {status}. Retrying...");
                continue;
            }

            // need to reauth
            if status.as_u16() == 401 {
                self.auth().await;
                continue;
            }

            // rate limit
            if status.as_u16() == 429 {
                println!("Request to {url} got rate limited");
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                continue;
            }
            println!("Reponse status {}", status);
            println!("Response body {}", text);
            panic!("Failed a request :(");
        }
        panic!("Failed after several retries :(");
    }

    async fn get_and_read(&self, suffix: &str, params: &HashMap<&str, String>) -> serde_json::Value {
        let pointer_json = self.get_with_retry(format!("{BASEURL}{suffix}"), params).await;
        return self.get_with_retry(String::from(pointer_json["link"].as_str().unwrap()), params).await;
    }

    async fn get_and_read_chunked(&self, suffix: &str, params: &HashMap<&str, String>) -> serde_json::Value {
        let pointer_json = self.get_with_retry(format!("{BASEURL}{suffix}"), params).await;
        let chunk_info = &pointer_json["data"]["chunk_info"];
        let base_url_res= &chunk_info["base_download_url"].as_str();

        let mut result_array = serde_json::Value::Array([].to_vec());
        if base_url_res.is_none() {
            return result_array;
        }

        let base_url = base_url_res.unwrap();

        for file in chunk_info["chunk_file_names"].as_array().unwrap() {
            let suffix = file.as_str().unwrap();
            let mut partial_result = self.get_with_retry(format!("{base_url}{suffix}"), &HashMap::new()).await;
            result_array.as_array_mut().unwrap().append(partial_result.as_array_mut().unwrap());
        }

        return result_array;
    }

    async fn get_member_since_year(&self, cust_id: i64) -> i32 {
        let params = HashMap::from([
            ("cust_ids", cust_id.to_string())
        ]);

        let res = self.get_and_read("/data/member/get", &params).await;

        return res["members"][0]["member_since"].as_str().unwrap()[0..4].parse::<i32>().unwrap();
    }

    async fn search_series(&self, cust_id: Option<i64>, year: i32, quarter: i32, week: Option<i32>) -> serde_json::Value {
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
        return self.get_and_read_chunked("/data/results/search_series", &params).await;
    }

    async fn find_subsessions_for_driver(&self, cust_id: i64) -> Vec<i64> {
        let member_since_year = self.get_member_since_year(cust_id).await;

        let mut series = serde_json::Value::Array([].to_vec());
        for year in member_since_year..2023+1 {
            for quarter in 1..4+1 {
                println!("Query {year}s{quarter}");
                let mut series_q = self.search_series(Some(cust_id), year, quarter, None).await;
                series.as_array_mut().unwrap().append(series_q.as_array_mut().unwrap());
            }
        }

        return series.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
    }

    async fn find_subsessions_for_season(&self, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
        let series = self.search_series(None, year, quarter, week).await;
        return series.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
    }

    async fn get_cust_id(&self, driver_name: &String) -> i64 {
        let res = self.get_and_read("/data/lookup/drivers", &HashMap::from([
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

    async fn lookup_driver(&self, cust_id: i64) -> serde_json::Value {
        return self.get_and_read("/data/lookup/drivers", &HashMap::from([
            ("cust_id", cust_id.to_string())
        ])).await;
    }

    pub async fn get_member_profile(&self, cust_id: i64) -> serde_json::Value {
        return self.get_and_read("/data/member/profile", &HashMap::from([
            ("cust_id", cust_id.to_string())
        ])).await;
    }


    pub async fn get_subsession(&self, subsession_id: i64) -> serde_json::Value {
        return self.get_and_read("/data/results/get", &HashMap::from([
            ("subsession_id", subsession_id.to_string())
        ])).await;
    }

    pub async fn auth(&self) {
        let user = std::env::var("IRACING_USER").unwrap();
        let token = std::env::var("IRACING_TOKEN").unwrap();

        let body = HashMap::from([
            ("email", user),
            ("password", token)
        ]);

        let response = self.client.post(format!("{}/auth", BASEURL)).json(&body).send().await.unwrap();
        assert!(response.status() == reqwest::StatusCode::OK);
    }
}

fn filter_non_cached(subsessions: Vec<i64>) -> Vec<i64> {
    let len1 = subsessions.len();
    let res: Vec<_> = subsessions.into_iter().filter(|ses| !crate::db::is_session_cached(*ses)).collect();
    let len2 = res.len();
    println!("Non-cached sessions {len2}/{len1}");
    return res;
}

async fn find_non_cached_subsessions_for_driver(client: &mut IRacingClient, cust_id: i64) -> Vec<i64> {
    return filter_non_cached(client.find_subsessions_for_driver(cust_id).await);
}

async fn find_non_cached_subsessions_for_season(client: &mut IRacingClient, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
    return filter_non_cached(client.find_subsessions_for_season(year, quarter, week).await);
}

async fn sync_subsession(client: &mut IRacingClient, subsession_id: i64, prefix: &str) {
    if crate::db::is_session_cached(subsession_id) {
        return;
    }

    println!("{prefix}Syncing session {subsession_id}");

    let res = client.get_subsession(subsession_id).await;

    crate::db::write_cached_session_json(subsession_id, &res);
}

async fn sync_subsessions(client: &mut IRacingClient, subsession_ids: &Vec<i64>) {

    let len = subsession_ids.len();
    println!("Syncing {len} subsessions");

    // Tried concurent stuff. Failed
    // let results = stream::iter(subsession_ids).map(|subsession_id| {
    //     let mut client = client.clone();
    //     async move {
    //         sync_subsession(&mut client, *subsession_id, "parallel").await;
    //     }
    // }).buffer_unordered(10);

    // results.collect::<Vec<()>>().await;

    let start = Instant::now();
    for (i, subsession_id) in subsession_ids.into_iter().enumerate() {
        let elapsed_secs = start.elapsed().as_secs_f32();
        let rate = i as f32 / elapsed_secs;
        sync_subsession(client, *subsession_id, format!("{i}/{len} {rate:.2}/s ").as_str()).await;
    }
}

fn add_subsessions_to_db(subsession_ids: &Vec<i64>) {
    let mut con = crate::db::create_db_connection();
    let mut tx = con.transaction().unwrap();
    {
        let mut ctx = crate::db::create_db_context(&mut tx);

        for subsession_id in subsession_ids {
            crate::db::add_session_to_db_from_cache(&mut ctx, *subsession_id);
        }
    }

    tx.commit().unwrap();
}

pub async fn sync_track_infos(client: &mut IRacingClient) {
    let data = client.get_and_read("/data/track/get", &HashMap::new()).await;
    crate::db::write_cached_track_infos_json(&data);
}

pub async fn sync_car_infos(client: &mut IRacingClient) {
    let data = client.get_and_read("/data/car/get", &HashMap::new()).await;
    crate::db::write_cached_car_infos_json(&data);
}

pub async fn sync_cust_ids_to_db(client: &mut IRacingClient, cust_ids: &Vec<i64>) {
    let mut subsession_ids = HashSet::<i64>::new();

    for cust_id in cust_ids {
        subsession_ids.extend(&mut find_non_cached_subsessions_for_driver(client, *cust_id).await.iter());
    }

    let subsession_ids_vec = Vec::from_iter(subsession_ids.into_iter());
    sync_subsessions(client, &subsession_ids_vec).await;
    add_subsessions_to_db(&subsession_ids_vec);
}

pub async fn sync_drivers_to_db(client: &mut IRacingClient, driver_names: &Vec<String>) {
    let mut cust_ids = Vec::new();

    for driver_name in driver_names {
        let cust_id = client.get_cust_id(driver_name).await;
        println!("{driver_name} -> {cust_id}");
        cust_ids.push(cust_id)
    }
    sync_cust_ids_to_db(client, &cust_ids).await;
}

pub async fn sync_season_to_db(client: &mut IRacingClient, year: i32, quarter: i32, week: Option<i32>) {
    let subsession_ids = find_non_cached_subsessions_for_season(client, year, quarter, week).await;
    sync_subsessions(client, &subsession_ids).await;
    add_subsessions_to_db(&subsession_ids);
}