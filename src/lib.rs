pub mod api;
pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod server;
pub mod services;
pub mod utils;
pub mod views;

#[macro_use]
extern crate diesel;

use crate::server::Context;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use std::error::Error;
use std::sync::Arc;
use warp::{
    host::Authority,
    http::{HeaderValue, StatusCode},
    reject, Filter, Rejection, Reply,
};

/// An API error serializable to JSON.
#[derive(Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct TesterJson {
    testing: u16,
}

pub async fn is_static(subdomain: Arc<Vec<String>>) -> Result<(), warp::Rejection> {
    log::info!("{:?}", subdomain);
    if subdomain.len() > 0 && subdomain[0] == "www" {
        Ok(())
    } else {
        Err(warp::reject())
    }
}

pub fn with_subdomain() -> warp::filters::BoxedFilter<(Arc<Vec<String>>,)> {
    warp::header::value("host")
        .map(move |value: HeaderValue| {
            // convert HeaderValue to String and split port if provided
            let splv: Vec<&str> = value.to_str().unwrap().split(":").collect();

            // split hostname
            let splv_2: Vec<String> = splv
                .first()
                .unwrap()
                .split(".")
                .map(|s: &str| String::from(s))
                .collect();

            Arc::<Vec<String>>::new(splv_2).clone()
        })
        .boxed()
}

pub async fn handle_final_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    log::error!("{}, {}", code, message);

    let json = warp::reply::html(views::error::error(code, message));
    Ok(warp::reply::with_status(json, code))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        Err(err)
    } else {
        let mut code = StatusCode::ACCEPTED;
        let mut message = String::from("Super unknown error??");

        if err.is_not_found() {
            code = StatusCode::NOT_FOUND;
            message = String::from("NOT_FOUND");
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = match e.source() {
                Some(cause) => format!("BAD_REQUEST: {cause}"),
                None => String::from("BAD_REQUEST"),
            };
            code = StatusCode::BAD_REQUEST;
        } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
            code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
            message = String::from("UNSUPPORTED_MEDIA_TYPE");
        } else {
            // We should have expected this... Just log and say its a 500
            error!("unhandled rejection: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = String::from("UNHANDLED_REJECTION");
        }
        log::error!("{}, {}", code, message);

        let json = warp::reply::html(views::error::error(code, &message));
        Ok(warp::reply::with_status(json, code))
    }
}
