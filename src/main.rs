mod filters;
mod mailer;
mod routes;

use crate::mailer::SmtpConnectionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tokio::fs;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RestApiConfig {
    pub api_keys: HashMap<String, SmtpConnectionOptions>,
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        unsafe { env::set_var("RUST_LOG", "rest_smtp=info") };
    }
    pretty_env_logger::init();

    let api_config_file =
        env::var_os("API_CONFIG_FILE").unwrap_or_else(|| "./api_config.json".parse().unwrap());

    println!("Loading api config from file {:?}", api_config_file.clone());

    let api_config_data = fs::read_to_string(api_config_file.clone())
        .await
        .expect("Could not read file api_config.json file");
    let api_config: RestApiConfig = serde_json::from_str(&api_config_data)
        .unwrap_or_else(|_| panic!("Could not parse {:?} contents", api_config_file.clone()));

    let api_routes = filters::rest_smtp_api(api_config);
    let api_routes = api_routes.with(warp::log("rest_smtp"));

    warp::serve(api_routes).run(([0, 0, 0, 0], 9002)).await;
}
