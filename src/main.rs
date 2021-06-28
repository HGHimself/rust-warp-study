use rust_warp_study::{
    handlers::hello_handler,
    routes::hello_route,
    telemetry::{get_subscriber, init_subscriber,init_tracing_logger},
};
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    init_tracing_logger();

    let hello = hello!();
    let end = hello
        .with(warp::trace::named("hello"))
        .with(warp::trace::request());

    warp::serve(end).run(([127, 0, 0, 1], 3030)).await;
}
