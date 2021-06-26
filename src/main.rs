#![deny(warnings)]
use rust_warp_study::{handlers::hello_handler, routes::hello_route};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let hello = hello!();
    let end = hello
        .with(warp::trace::named("hello"))
        .with(warp::trace::request());
    warp::serve(end).run(([127, 0, 0, 1], 3030)).await;
}
