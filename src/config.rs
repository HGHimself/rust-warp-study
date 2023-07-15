use dotenv::dotenv;
use log::info;
use std::env;

// how many requests we will allow to process at once
// all others wait their turn
const MAX_INFLIGHT_REQUESTS: usize = 100;
// how many connections can be open an running at one time
// the rest wait until a permit opens up
const MAX_CONNS: usize = 100;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_addr: String,
    pub max_conn: usize,
    pub max_reqs: usize,
    pub is_mocking: bool,
    pub db_path: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

impl Config {
    pub fn new(is_mocking: bool) -> Self {
        info!("🤖 Configuring the application!");
        dotenv().ok();

        // app fields
        let app_host = env::var("HOST").expect("HOST must be set");
        let app_port = env::var("PORT").expect("PORT must be set");
        let app_addr = format!("{}:{}", app_host, app_port);

        let max_conn = match env::var("MAX_CONN") {
            Ok(mc) => mc.parse::<usize>().expect("MAX_CONN must be an integer"),
            Err(_) => MAX_CONNS,
        };

        let max_reqs = match env::var("MAX_REQS") {
            Ok(mr) => mr.parse::<usize>().expect("MAX_REQS must be an integer"),
            Err(_) => MAX_INFLIGHT_REQUESTS,
        };

        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // prepare tls if necessary
        let tls = env::var("ENABLE_TLS")
            .expect("ENABLE_TLS must be set")
            .parse()
            .expect("ENABLE_TLS must be true or false");

        let cert_path;
        let key_path;
        if tls {
            cert_path = Some(env::var("CERT_PATH").expect("CERT_PATH must be set"));
            key_path = Some(env::var("KEY_PATH").expect("KEY_PATH must be set"));
        } else {
            cert_path = None;
            key_path = None;
        }

        Config {
            app_addr,
            max_conn,
            max_reqs,
            is_mocking,
            db_path,
            tls,
            cert_path,
            key_path,
        }
    }
}

#[cfg(feature = "mocks")]
pub fn generate_mocking_config() -> Config {
    Config::new(true)
}

pub fn generate_config() -> Config {
    Config::new(false)
}
