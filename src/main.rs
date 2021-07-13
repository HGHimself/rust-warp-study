use env_logger::Env;
use rust_warp_study::{handlers::hello_handler, routes::hello_route};
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let hello = hello!();
    let end = hello.with(warp::log("hello"));

    warp::serve(end).run(([127, 0, 0, 1], 3030)).await;
}
