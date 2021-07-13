use crate::{services::hello_service, ErrorMessage};
use log::{error, info};
use std::convert::Infallible;
use std::error::Error;
use warp;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

pub async fn hello(name: u64) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    let reply = format!("Hello, {}!", name);
    println!("{}", &reply);
    Ok(warp::reply::html(reply))
}

pub async fn hello_panic(name: u64) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received hello request for name: {}", name);
    let message;
    let code;
    match hello_service::go_crazy() {
        Err(e) => {
            code = StatusCode::BAD_REQUEST;
            error!("Erroring: {}", name);
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: e.into(),
            });

            Ok(warp::reply::with_status(json, code))
        }
        Ok(_) => {
            code = StatusCode::OK;
            message = "Hello, {}!";
            let json = warp::reply::json(&ErrorMessage {
                code: code.as_u16(),
                message: message.into(),
            });

            Ok(warp::reply::with_status(json, code))
        }
    }
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
