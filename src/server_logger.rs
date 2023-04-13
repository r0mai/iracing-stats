use rocket::{fairing::Fairing, Request};
use std::fs::{OpenOptions, File};
use std::path::PathBuf;
use std::io::prelude::*;

pub struct ServerLogger {
    log_file_path: PathBuf
}

impl ServerLogger {
    pub fn new(log_file_path: PathBuf) -> Self {
        return Self { log_file_path };
    }
}

#[rocket::async_trait]
impl Fairing for ServerLogger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "ServerLogger",
            kind: rocket::fairing::Kind::Request
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        let uri = request.uri();
        let now = chrono::offset::Utc::now();
        let mut log_file = OpenOptions::new().append(true).create(true).open(&self.log_file_path).unwrap();
        writeln!(log_file, "{:?} request to {}", now, uri).ok(); // ignore error
        log_file.flush().ok(); // ignore error
    }
}
