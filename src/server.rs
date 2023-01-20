use std::collections::HashMap;

use rocket::fs::FileServer;

use crate::category_type::CategoryType;
use crate::db::{
    query_irating_history,
    query_track_car_usage_matrix,
    query_track_usage,
    query_car_usage, query_driver_stats
};
use serde_json::{Value, json};

#[get("/api/v1/irating-history?<driver_name>&<category>")]
async fn api_v1_irating_history(driver_name: String, category: Option<String>) -> Value {
    let category_type = match category {
        Some(str) => CategoryType::from_string(str.as_str()).unwrap_or(CategoryType::Road),
        None => CategoryType::Road
    };

    return query_irating_history(&driver_name, category_type);
}

#[get("/api/v1/track-usage-stats?<driver_name>")]
async fn api_v1_track_usage_stats(driver_name: String) -> Value {
    let raw_data = query_track_usage(&driver_name); 

    let values: Vec<Value> = raw_data.iter().map(|data| json!({
        "track_name": data.track_name,
        "time": data.time,
        "laps": data.laps,
        "distance": data.distance,
    })).collect();

    return Value::Array(values);
}

#[get("/api/v1/car-usage-stats?<driver_name>")]
async fn api_v1_car_usage_stats(driver_name: String) -> Value {
    let raw_data = query_car_usage(&driver_name); 

    let values: Vec<Value> = raw_data.iter().map(|data| json!({
        "car_name": data.car_name,
        "time": data.time,
        "distance": data.distance,
    })).collect();

    return Value::Array(values);
}

#[get("/api/v1/car-track-usage-stats?<driver_name>")]
async fn api_v1_car_track_usage_stats(driver_name: String) -> Value {
    let raw_data = query_track_car_usage_matrix(&driver_name); 

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

    return json!({
        "matrix": matrix,
        "cars": cars,
        "tracks": tracks
    });
}

#[get("/api/v1/driver-stats?<driver_name>")]
async fn api_v1_driver_stats(driver_name: String) -> Value {
    return query_driver_stats(&driver_name);
}

pub async fn start_rocket_server() {
    let _result = rocket::build()
        .mount("/", routes![
            api_v1_irating_history,
            api_v1_car_track_usage_stats,
            api_v1_track_usage_stats,
            api_v1_car_usage_stats,
            api_v1_driver_stats,
        ])
        .mount("/static", FileServer::from("static"))
        .launch().await.unwrap();
}