use rocket::{fairing::Fairing, Request};

pub struct ResponseTimer {}

impl ResponseTimer {
    pub fn new() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for ResponseTimer {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "ResponseTimer",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        request.local_cache(|| std::time::Instant::now());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, _response: &mut rocket::Response<'r>) {
        let start_time = request.local_cache(|| std::time::Instant::now());
        let end_time = start_time.elapsed();

        // response.set_raw_header("X-Response-Time", format!("{:.2?}", end_time));
        if let Some(query) = request.uri().query() {
            println!("Timing: {} -- {}?{}", end_time.as_millis(), request.uri().path(), query);
        }
    }
}
