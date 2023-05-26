pub mod api;
pub mod config;
pub mod handlers;
pub mod routes;
pub mod server;
pub mod services;

use crate::services::webauthn::Webauthn;
use log::error;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use std::error::Error;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

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

#[derive(Debug)]
pub struct NotEven;
impl reject::Reject for NotEven {}

#[derive(Clone)]
pub struct Config {
    num: u64,
}

pub fn with_config(config: Arc<Config>) -> warp::filters::BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}

pub fn with_webauthn(webauthn: Arc<Webauthn>) -> warp::filters::BoxedFilter<(Arc<Webauthn>,)> {
    warp::any().map(move || webauthn.clone()).boxed()
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = String::from("NOT_FOUND");
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        message = match e.source() {
            Some(cause) => format!("BAD_REQUEST: {cause}"),
            None => String::from("BAD_REQUEST"),
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = String::from("METHOD_NOT_ALLOWED");
    } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = String::from("UNSUPPORTED_MEDIA_TYPE");
    } else {
        // We should have expected this... Just log and say its a 500
        error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("UNHANDLED_REJECTION");
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message,
    });

    Ok(warp::reply::with_status(json, code))
}
