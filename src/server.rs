use std::collections::HashMap;

use rocket::fs::FileServer;
use rocket::State;

use crate::category_type::CategoryType;
use crate::driverid::DriverId;
use crate::db::{
    query_irating_history,
    query_track_car_usage_matrix,
    query_track_usage,
    query_car_usage,
    query_driver_stats,
};
use serde_json::{Value, json};
use crate::iracing_client::IRacingClient;

#[get("/api/v1/irating-history?<driver_name>&<cust_id>&<category>")]
async fn api_v1_irating_history(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    category: Option<String>) -> Option<Value>
{
    let category_type = match category {
        Some(str) => CategoryType::from_string(str.as_str()).unwrap_or(CategoryType::Road),
        None => CategoryType::Road
    };

    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        return Some(query_irating_history(&driver_id, category_type));
    } else {
        return None;
    }

}

#[get("/api/v1/track-usage-stats?<driver_name>&<cust_id>")]
async fn api_v1_track_usage_stats(
    driver_name: Option<String>,
    cust_id: Option<i64>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let raw_data = query_track_usage(&driver_id);

        let values: Vec<Value> = raw_data.iter().map(|data| json!({
            "track_name": data.track_name,
            "time": data.time,
            "laps": data.laps,
            "distance": data.distance,
        })).collect();

        return Some(Value::Array(values));
    } else {
        return None;
    }
}

#[get("/api/v1/car-usage-stats?<driver_name>&<cust_id>")]
async fn api_v1_car_usage_stats(
    driver_name: Option<String>,
    cust_id: Option<i64>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let raw_data = query_car_usage(&driver_id); 

        let values: Vec<Value> = raw_data.iter().map(|data| json!({
            "car_name": data.car_name,
            "time": data.time,
            "distance": data.distance,
        })).collect();

        return Some(Value::Array(values));
    } else {
        return None;
    }
}

#[get("/api/v1/car-track-usage-stats?<driver_name>&<cust_id>")]
async fn api_v1_car_track_usage_stats(
    driver_name: Option<String>,
    cust_id: Option<i64>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let raw_data = query_track_car_usage_matrix(&driver_id); 

        let mut car_idxs = HashMap::new();
        let mut track_idxs = HashMap::new();

        let mut car_idx = 0;
        let mut track_idx = 0;

        for data in raw_data.iter() {
            if !car_idxs.contains_key(&data.car_name) {
                car_idxs.insert(data.car_name.clone(), car_idx);
                car_idx += 1;
            }
            if !track_idxs.contains_key(&data.track_name) {
                track_idxs.insert(data.track_name.clone(), track_idx);
                track_idx += 1;
            }
        }

        let car_count = car_idxs.len();
        let track_count = track_idxs.len();

        // matrix[track][car]
        let mut matrix = vec![vec![json!({}); car_count]; track_count];

        for data in raw_data.into_iter() {
            matrix[track_idxs[&data.track_name]][car_idxs[&data.car_name]] = json!({
                "time": data.time,
                "laps": data.laps
            });
        }

        let mut cars = vec![String::new(); car_count];
        let mut tracks = vec![String::new(); track_count];

        for (name, idx) in car_idxs {
            cars[idx] = name;
        }

        for (name, idx) in track_idxs {
            tracks[idx] = name;
        }

        return Some(json!({
            "matrix": matrix,
            "cars": cars,
            "tracks": tracks
        }));
    } else {
        return None;
    }
}

fn transform_member_info(member_profile: &Value) -> Value {
    fn find_category(array: &Value, category: CategoryType) -> &Value {
        for e in array.as_array().unwrap() {
            if e["category_id"].as_i64().unwrap() == category as i64 {
                return e;
            }
        }
        return &Value::Null;
    }

    if member_profile.is_null() {
        return Value::Null;
    }

    let licenses = &member_profile["member_info"]["licenses"];
    let result = json!({
        "oval": find_category(&licenses, CategoryType::Oval),
        "road": find_category(&licenses, CategoryType::Road),
        "dirt_oval": find_category(&licenses, CategoryType::DirtOval),
        "dirt_road": find_category(&licenses, CategoryType::DirtRoad),
    });
    return result;
}

#[get("/api/v1/driver-stats?<driver_name>&<cust_id>")]
async fn api_v1_driver_stats(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    iracing_client: &State<IRacingClient>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        if let Some(stats) = query_driver_stats(&driver_id) {
            let member_profile = iracing_client.get_member_profile(stats.cust_id).await;

            return Some(json!({
                "name": stats.name,
                "time": stats.time,
                "laps": stats.laps,
                "distance": stats.distance,
                "licenses": transform_member_info(&member_profile)
            }));
        }
    }
    return None;
}

pub async fn start_rocket_server() {
    let _result = rocket::build()
        // .mount("/static", FileServer::from("static"))
        .mount("/", FileServer::from("site/build"))
        .mount("/", routes![
            api_v1_irating_history,
            api_v1_car_track_usage_stats,
            api_v1_track_usage_stats,
            api_v1_car_usage_stats,
            api_v1_driver_stats,
        ])
        .manage(IRacingClient::new())
        .launch().await.unwrap();
}