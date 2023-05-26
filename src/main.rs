use env_logger::Env;
use rust_warp_study::{
    config::Config,
    server::{serve, ServerProps},
    services::webauthn::Webauthn,
};

use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("🐙 Booting up the API!");

    let config = Arc::new(Config::new(false));
    let webauthn = Arc::new(Webauthn::new());

    serve(config, webauthn).await;
}
