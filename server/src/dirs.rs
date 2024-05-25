use lazy_static::lazy_static;
use std::{path::PathBuf, path::Path, env};

const BASE_DIR_ENV_VAR: &str = "IRACING_STATS_BASE_DIR";
const STATIC_DIR_ENV_VAR: &str = "IRACING_STATS_STATIC_DIR";

pub fn get_base_dir() -> &'static Path {
    lazy_static! {
        static ref BASE_DIR: PathBuf = PathBuf::from(
            match env::var(BASE_DIR_ENV_VAR) {
                Ok(value) => value,
                Err(_error) => ".".to_owned()
            }
        );
    }
    return BASE_DIR.as_path();
}

pub fn get_static_dir() -> &'static Path {
    lazy_static! {
        static ref STATIC_DIR: PathBuf = PathBuf::from(
            match env::var(STATIC_DIR_ENV_VAR) {
                Ok(value) => value,
                Err(_error) => ".".to_owned()
            }
        );
    }
    return STATIC_DIR.as_path();
}