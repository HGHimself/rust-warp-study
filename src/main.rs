use rust_warp_study::{config::Config, server::serve};
use tracing_subscriber::fmt::format::FmtSpan;

use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "rust_warp_study=debug,tower_http=debug,warp=debug".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // // Record an event when each span closes. This can be used to time our
        // // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    log::info!("🐙 Booting up the API!");

    let config = Arc::new(Config::new(false));

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    serve(socket_address, config).await.expect("server error");
}
