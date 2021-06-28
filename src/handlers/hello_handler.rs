use tracing::info;
use crate::services::hello_service;
use warp;

pub async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn hello_panic(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    hello_service::go_crazy();
    let reply = format!("{{\"message\": \"Hello, {}!\"}}", name);
    Ok(warp::reply::json(&reply))
}
