use crate::{Config, ErrorMessage, NotEven, TesterJson};
use log::info;
use std::{convert::Infallible, sync::Arc, time::Duration};
use warp::{http::StatusCode, Rejection};

pub async fn hello(name: u64, config: Arc<Config>) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    let reply = format!("Hello, {}!", name * config.num);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn hello_post(tester: TesterJson) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello post request for name: {:?}", tester);
    let reply = format!("Hello, {}!", tester.testing);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn hello_rejection(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    let code;
    let message;

    if let Some(NotEven) = err.find() {
        message = String::from("NOT_EVEN");
        code = StatusCode::BAD_REQUEST;

        let json = warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            message: message,
        });

        Ok(warp::reply::with_status(json, code))
    } else {
        Err(err)
    }
}

pub async fn sleepy(seconds: u64) -> Result<impl warp::Reply, Infallible> {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    Ok(format!("I waited {} seconds!", seconds))
}
