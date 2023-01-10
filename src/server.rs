use rocket::serde;

use crate::db::query_irating_history;
use serde_json::Value;

#[get("/api/v1/irating-history?<driver_name>")]
async fn api_v1_irating_history(driver_name: String) -> Value {
    return query_irating_history(&driver_name);
}

pub async fn start_rocket_server() {
    let _result = rocket::build()
        .mount("/", routes![
            api_v1_irating_history
        ])
        .launch().await.unwrap();
}