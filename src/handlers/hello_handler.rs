use log::info;
use warp;

pub async fn hello_prime(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn hello(name: u64) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    let reply = format!("{{\"message\": \"Hello, {}!\"}}", name);
    Ok(warp::reply::json(&reply))
}
