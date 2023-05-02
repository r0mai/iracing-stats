use std::{
    collections::HashMap,
    collections::HashSet,
    sync::atomic::AtomicI64,
    sync::atomic::Ordering, thread::current
};
use chrono::{Utc, DateTime, Days, Datelike, NaiveDateTime, NaiveDate, FixedOffset, TimeZone, NaiveTime};
use serde_json;
use reqwest::{self, Client, header::HeaderValue};
use std::time::Instant;
use lazy_static::lazy_static;

use crate::db::query_all_site_team_members;

const BASEURL: &str = "https://members-ng.iracing.com";
const CURRENT_YEAR: i32 = 2023;
const CURRENT_QUARTER: i32 = 2;

pub struct IRacingClient {
    pub client: Client,
    pub rate_limit_limit: AtomicI64,
    pub rate_limit_remaining: AtomicI64,
    pub rate_limit_reset: AtomicI64,
}

fn cached_now() -> DateTime<Utc> {
    lazy_static! {
        static ref NOW: DateTime<Utc> = Utc::now();
    }
    return *NOW;
}

fn to_api_date_string(date: &DateTime<Utc>) -> String {
    return date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
}

fn extract_subsession_ids_from_response(response: &serde_json::Value) -> Vec<i64> {
    return response.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
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

    async fn get_with_retry(&self, url: String, params: &HashMap<&str, String>) -> Option<serde_json::Value> {
        for _ in 0..10 {
            let response_res = self.client.get(&url).query(&params).send().await;
            if let Err(error) = response_res {
                println!("Error {error} while requesting {url}");
                continue;
            }
            let response = response_res.unwrap();
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
                return Some(serde_json::from_str(&text).unwrap());
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

            // unauthorized to view session
            if status.as_u16() == 403 {
                println!("Request to {url} was unauthorized (403)");
                return None;
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

    async fn get_and_read(&self, suffix: &str, params: &HashMap<&str, String>) -> Option<serde_json::Value> {
        let pointer_json = self.get_with_retry(format!("{BASEURL}{suffix}"), params).await?;
        return self.get_with_retry(String::from(pointer_json["link"].as_str().unwrap()), params).await;
    }

    async fn get_and_read_chunked(&self, suffix: &str, params: &HashMap<&str, String>) -> Option<serde_json::Value> {
        let pointer_json = self.get_with_retry(format!("{BASEURL}{suffix}"), params).await?;
        let chunk_info = &pointer_json["data"]["chunk_info"];
        let base_url_res= &chunk_info["base_download_url"].as_str();

        let mut result_array = serde_json::Value::Array([].to_vec());
        if base_url_res.is_none() {
            return Some(result_array);
        }

        let base_url = base_url_res.unwrap();

        for file in chunk_info["chunk_file_names"].as_array().unwrap() {
            let suffix = file.as_str().unwrap();
            let mut partial_result = self.get_with_retry(format!("{base_url}{suffix}"), &HashMap::new()).await?;
            result_array.as_array_mut().unwrap().append(partial_result.as_array_mut().unwrap());
        }

        return Some(result_array);
    }

    async fn get_member_since_date(&self, cust_id: i64) -> DateTime<Utc> {
        let params = HashMap::from([
            ("cust_ids", cust_id.to_string())
        ]);

        let res = self.get_and_read("/data/member/get", &params).await.unwrap();

        let date_str = res["members"][0]["member_since"].as_str().unwrap();

        // TODO this can be probably done without involving timezones at all
        let tz_utc = FixedOffset::east_opt(0).unwrap(); // hope this is UTC

        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap(); // midnight?
        let naive_date_time = NaiveDateTime::new(naive_date, naive_time);

        return tz_utc.from_local_datetime(&naive_date_time).unwrap().with_timezone(&Utc);
    }

    // a.k.a series list
    async fn get_season_list(&self, year: i32, quarter: i32) -> serde_json::Value {
        let params = HashMap::from([
            ("season_year", year.to_string()),
            ("season_quarter", quarter.to_string()),
        ]);
        return self.get_and_read("/data/season/list", &params).await.unwrap();
    }

    async fn get_all_season_list(&self) -> serde_json::Value {
        let mut seasons = Vec::new();
        for year in 2008..=CURRENT_YEAR {
            let last_quarter = if year == CURRENT_YEAR { CURRENT_QUARTER } else { 4 };
            for quarter in 1..=last_quarter {
                println!("Syncing season {year}s{quarter}");
                let mut current_season_list = self.get_season_list(year, quarter).await;
                seasons.append(current_season_list["seasons"].as_array_mut().unwrap());

            }
        }
        return serde_json::Value::Array(seasons);
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
        return self.get_and_read_chunked("/data/results/search_series", &params).await.unwrap();
    }

    async fn search_hosted(&self, cust_id: i64, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> serde_json::Value {
        let params = HashMap::from([
            ("cust_id", cust_id.to_string()),
            ("start_range_begin", to_api_date_string(start_date)),
            ("start_range_end", to_api_date_string(end_date)),
        ]);
        return self.get_and_read_chunked("/data/results/search_hosted", &params).await.unwrap();
    }

    // return subsession_ids may contain duplicates
    async fn find_subsessions_for_driver(&self, cust_id: i64, partial: bool) -> Vec<i64> {
        let mut seasons = Vec::new();
        let start_date;
        if partial {
            seasons.push((CURRENT_YEAR, CURRENT_QUARTER));
            start_date = cached_now().checked_sub_days(Days::new(10)).unwrap();
        } else {
            let member_since = self.get_member_since_date(cust_id).await;
            let member_since_year = member_since.year();
            for year in member_since_year..=CURRENT_YEAR {
                let last_quarter = if year == CURRENT_YEAR { CURRENT_QUARTER } else { 4 };
                for quarter in 1..=last_quarter {
                    seasons.push((year, quarter));
                }
            }
            start_date = member_since;
        }

        let mut subsession_ids = Vec::new();

        // official
        for (year, quarter) in seasons {
            println!("Query official {year}s{quarter}");
            let series_q = self.search_series(Some(cust_id), year, quarter, None).await;
            let mut new_ids = extract_subsession_ids_from_response(&series_q);
            subsession_ids.append(&mut new_ids);
        }

        // hosted
        let mut current_date = start_date;
        let last_date = cached_now().checked_add_days(Days::new(1)).unwrap();
        while current_date < last_date {
            // max range allowed is 90. be safe with 89
            let next_date = current_date.checked_add_days(Days::new(89)).unwrap();

            println!("Query hosted {current_date} -> {next_date}");
            let hosted_q = self.search_hosted(cust_id, &current_date, &next_date).await;
            let mut new_ids = extract_subsession_ids_from_response(&hosted_q);
            subsession_ids.append(&mut new_ids);
            
            current_date = next_date;
        }

        return subsession_ids;
    }

    async fn find_subsessions_for_season(&self, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
        let series = self.search_series(None, year, quarter, week).await;
        return series.as_array().unwrap().iter().map(|ses| ses["subsession_id"].as_i64().unwrap()).collect();
    }

    async fn get_cust_id(&self, driver_name: &String) -> i64 {
        let res = self.get_and_read("/data/lookup/drivers", &HashMap::from([
            ("search_term", driver_name.clone())
        ])).await.unwrap();
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
        ])).await.unwrap();
    }

    pub async fn get_member_profile(&self, cust_id: i64) -> serde_json::Value {
        return self.get_and_read("/data/member/profile", &HashMap::from([
            ("cust_id", cust_id.to_string())
        ])).await.unwrap();
    }


    pub async fn get_subsession(&self, subsession_id: i64) -> Option<serde_json::Value> {
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

async fn find_non_cached_subsessions_for_driver(client: &mut IRacingClient, cust_id: i64, partial: bool) -> Vec<i64> {
    return filter_non_cached(client.find_subsessions_for_driver(cust_id, partial).await);
}

async fn find_non_cached_subsessions_for_season(client: &mut IRacingClient, year: i32, quarter: i32, week: Option<i32>) -> Vec<i64> {
    return filter_non_cached(client.find_subsessions_for_season(year, quarter, week).await);
}

async fn sync_subsession(client: &mut IRacingClient, subsession_id: i64, prefix: &str) -> bool {
    if crate::db::is_session_cached(subsession_id) {
        return true;
    }

    println!("{prefix}Syncing session {subsession_id}");

    if let Some(res) = client.get_subsession(subsession_id).await {
        crate::db::write_cached_session_json(subsession_id, &res);
        return true;
    }
    return false;
}

async fn sync_subsessions(client: &mut IRacingClient, subsession_ids: &Vec<i64>) -> Vec<i64> {

    let len = subsession_ids.len();
    println!("Syncing {len} subsessions");

    let mut synced_subsession_ids = Vec::new();

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
        let ip1 = i+1;

        // This can fail if we don't have permission to view the subsession
        let success = sync_subsession(client, *subsession_id, format!("{ip1}/{len} {rate:.2}/s ").as_str()).await;
        if success {
            synced_subsession_ids.push(*subsession_id);
        }
    }
    return synced_subsession_ids;
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

pub async fn sync_track_infos_to_db(client: &mut IRacingClient) {
    let data = client.get_and_read("/data/track/get", &HashMap::new()).await.unwrap();
    crate::db::write_cached_track_infos_json(&data);
    crate::db::rebuild_tracks_in_db();
}

pub async fn sync_car_infos_to_db(client: &mut IRacingClient) {
    let data = client.get_and_read("/data/car/get", &HashMap::new()).await.unwrap();
    crate::db::write_cached_car_infos_json(&data);
    crate::db::rebuild_cars_in_db();
}

pub async fn sync_season_infos_to_db(client: &mut IRacingClient) {
    let data = client.get_all_season_list().await;
    crate::db::write_cached_seasons_json(&data);
    crate::db::rebuild_seasons_in_db();
}

pub async fn sync_site_teams_to_db(client: &mut IRacingClient, partial: bool) {
    let mut con = crate::db::create_db_connection();
    let cust_ids = query_all_site_team_members(&mut con);
    if partial {
        sync_cust_ids_to_db(client, &Vec::new(), &cust_ids).await;
    } else {
        sync_cust_ids_to_db(client, &cust_ids, &Vec::new()).await;
    }
}

pub async fn sync_cust_ids_to_db(client: &mut IRacingClient, cust_ids: &Vec<i64>, cust_ids_partial: &Vec<i64>) {
    let mut subsession_ids = HashSet::<i64>::new();

    for cust_id in cust_ids {
        subsession_ids.extend(&mut find_non_cached_subsessions_for_driver(client, *cust_id, false).await.iter());
    }
    for cust_id in cust_ids_partial {
        subsession_ids.extend(&mut find_non_cached_subsessions_for_driver(client, *cust_id, true).await.iter());
    }

    let subsession_ids_vec = Vec::from_iter(subsession_ids.into_iter());
    let synced_subsession_ids = sync_subsessions(client, &subsession_ids_vec).await;
    add_subsessions_to_db(&synced_subsession_ids);
}

pub async fn sync_drivers_to_db(client: &mut IRacingClient, driver_names: &Vec<String>, driver_names_partial: &Vec<String>) {
    let mut cust_ids = Vec::new();
    let mut cust_ids_partial = Vec::new();

    for driver_name in driver_names {
        let cust_id = client.get_cust_id(driver_name).await;
        println!("{driver_name} -> {cust_id}");
        cust_ids.push(cust_id)
    }
    for driver_name in driver_names_partial {
        let cust_id = client.get_cust_id(driver_name).await;
        println!("{driver_name} -> {cust_id}");
        cust_ids_partial.push(cust_id)
    }
    sync_cust_ids_to_db(client, &cust_ids, &cust_ids_partial).await;
}

pub async fn sync_season_to_db(client: &mut IRacingClient, year: i32, quarter: i32, week: Option<i32>) {
    let subsession_ids = find_non_cached_subsessions_for_season(client, year, quarter, week).await;
    let synced_subsession_ids = sync_subsessions(client, &subsession_ids).await;
    add_subsessions_to_db(&synced_subsession_ids);
}