use std::env;
use std::collections::HashMap;

use rocket::fs::{FileServer, Options};
use rocket::State;

use rusqlite::Connection;

use crate::category_type::CategoryType;
use crate::driverid::DriverId;
use crate::db::{
    DbPool,
    CustomerName,
    create_r2d2_db_connection_pool,
    query_irating_history,
    query_track_car_usage_matrix,
    query_track_usage,
    query_car_usage,
    query_driver_stats,
    query_customer_names,
    query_customer_cust_ids,
    query_driver_sessions,
    query_track_data,
    query_car_data, query_site_team_members
};
use serde_json::{Value, json};
use crate::iracing_client::IRacingClient;

#[get("/api/v1/irating-history?<driver_name>&<cust_id>&<category>")]
async fn api_v1_irating_history(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    category: Option<String>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    let category_type = match category {
        Some(str) => CategoryType::from_string(str.as_str()).unwrap_or(CategoryType::Road),
        None => CategoryType::Road
    };

    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        return Some(query_irating_history(&con, &driver_id, category_type));
    } else {
        return None;
    }

}

#[get("/api/v1/track-usage-stats?<driver_name>&<cust_id>")]
async fn api_v1_track_usage_stats(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        let raw_data = query_track_usage(&con, &driver_id);

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
    cust_id: Option<i64>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        let raw_data = query_car_usage(&con, &driver_id); 

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
    cust_id: Option<i64>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        let raw_data = query_track_car_usage_matrix(&con, &driver_id); 

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
    iracing_client: &State<IRacingClient>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let stats_opt;
        {
            let con = db_pool.get().unwrap();
            stats_opt = query_driver_stats(&con, &driver_id);
        }
        if let Some(stats) = stats_opt {
            // let member_profile = iracing_client.get_member_profile(stats.cust_id).await;
            let member_profile = Value::Null;

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

#[get("/api/v1/driver-info?<driver_name>&<cust_id>")]
async fn api_v1_driver_info(
    driver_name: Option<String>,
    cust_id: Option<i64>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    if let Some(driver_id) = DriverId::from_params(driver_name, cust_id) {
        let con = db_pool.get().unwrap();
        let raw_data = query_driver_sessions(&con, &driver_id)?;

        let values: Vec<Value> = raw_data.iter().map(|data| json!({
            "subsession_id": data.subsession_id,
            "old_irating": data.old_irating,
            "new_irating": data.new_irating,
            "old_cpi": data.old_cpi,
            "new_cpi": data.new_cpi,
            "incidents": data.incidents,
            "laps_complete": data.laps_complete,
            "average_lap": data.average_lap,
            "finish_position_in_class": data.finish_position_in_class,
            "car_id": data.car_id,
            "track_id": data.track_id,
            "package_id": data.package_id,
            "license_category": data.license_category.to_db_type(),
            "start_time": data.start_time,
            "event_type": data.event_type.to_db_type(),
            "series_name": data.series_name,
            "simsession_number": data.simsession_number,
        })).collect();

        return Some(json!({
            "sessions": values
        }));
    } else {
        return None;
    }
}

#[get("/api/v1/track-car-data")]
async fn api_v1_track_car_data(db_pool: &State<DbPool>) -> Value {
    // TODO caching
    let con = db_pool.get().unwrap();

    let track_data = query_track_data(&con);
    let car_data = query_car_data(&con);

    let mut tracks = Vec::new();
    for track in track_data {
        tracks.push(json!({
            "package_id": track.package_id,
            "track_id": track.track_id,
            "track_name": track.track_name,
            "config_name": track.config_name,
            "track_config_length": track.track_config_length,
            "category": track.category.to_db_type()
        }));
    }

    let mut cars = Vec::new();
    for car in car_data {
        cars.push(json!({
            "car_id": car.car_id,
            "car_name": car.car_name,
            "car_name_abbreviated": car.car_name_abbreviated
        }));
    }

    return json!({
        "tracks": tracks,
        "cars": cars
    });
}

fn parse_team_customer_infos(con: &Connection, team: &String) -> Vec<CustomerName> {
    return query_site_team_members(con, team);
}

fn parse_drivers_customer_infos(drivers: &String) -> Option<Vec<CustomerName>> {
    let mut infos = Vec::new();
    for driver in drivers.split(";") {
        if driver.is_empty() {
            continue;
        }

        if driver.chars().nth(0).unwrap() == '$' {
            let cust_id_str: String = driver.chars().skip(1).collect();
            if let Ok(cust_id) = cust_id_str.parse::<i64>() {
                infos.push(CustomerName{name: "".to_owned(), cust_id})
            } else {
                return None;
            }
        } else {
            infos.push(CustomerName{name: driver.to_owned(), cust_id: -1})
        }
    }
    return Some(infos);
}

#[get("/api/v1/customers?<team>&<drivers>")]
async fn api_v1_customers(
    team: Option<String>,
    drivers: Option<String>,
    db_pool: &State<DbPool>) -> Option<Value>
{
    let con = db_pool.get().unwrap();

    let infos;
    if let Some(team) = team {
        infos = parse_team_customer_infos(&con, &team);
    } else if let Some(drivers) = drivers {
        infos = parse_drivers_customer_infos(&drivers)?;
    } else {
        return Option::None;
    }

    // fill out missing info
    let mut cust_ids = Vec::new();
    let mut names = Vec::new();
    let mut result = Vec::new();
    for info in infos.into_iter() {
        if info.name.is_empty() {
            cust_ids.push(info.cust_id);
        } else if info.cust_id == -1 {
            names.push(info.name);
        } else {
            result.push(info);
        }
    }

    result.append(&mut query_customer_names(&con, cust_ids));
    result.append(&mut query_customer_cust_ids(&con, names));

    let json_arr = result.iter().map(|name| {
        return json!({
            "name": name.name,
            "cust_id": name.cust_id
        });
    }).collect();

    return Some(Value::Array(json_arr));
}

#[get("/api/v1/customer-names?<cust_ids>")]
async fn api_v1_customer_names(
    cust_ids: String,
    db_pool: &State<DbPool>) -> Option<Value>
{
    let cust_id_strs = cust_ids.split(";");
    let mut cust_id_nums = vec![];
    for str in cust_id_strs {
        if let Ok(num) = str.parse::<i64>() {
            cust_id_nums.push(num);
        }
    }

    let con = db_pool.get().unwrap();
    let names = query_customer_names(&con, cust_id_nums);

    let result = names.iter().map(|name| {
        return json!({
            "name": name.name,
            "cust_id": name.cust_id
        });
    }).collect();

    return Some(Value::Array(result));
}

pub async fn start_rocket_server() {
    const SITE_DIR_ENV_VAR: &str = "IRACING_STATS_SITE_DIR";

    let db_pool = create_r2d2_db_connection_pool();

    let site_dir = match env::var(SITE_DIR_ENV_VAR) {
        Ok(value) => value,
        Err(_error) => "site/build".to_owned()
    };
    let _result = rocket::build()
        // .mount("/static", FileServer::from("static"))
        .mount("/iracing-stats", FileServer::new(site_dir, Options::Index))
        .mount("/", routes![
            api_v1_irating_history,
            api_v1_car_track_usage_stats,
            api_v1_track_usage_stats,
            api_v1_car_usage_stats,
            api_v1_driver_stats,
            api_v1_customers,
            api_v1_customer_names,
            api_v1_driver_info,
            api_v1_track_car_data
        ])
        .manage(IRacingClient::new())
        .manage(db_pool)
        .attach(crate::response_timer::ResponseTimer::new())
        .launch().await.unwrap();
}