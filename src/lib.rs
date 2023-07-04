#![feature(let_chains)]

pub mod api;
pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod server;
pub mod utils;
pub mod views;

#[macro_use]
extern crate diesel;

use log::error;
use server::Context;
use std::convert::Infallible;
use std::error::Error;
use std::sync::Arc;
use warp::{
    http::{HeaderValue, StatusCode},
    reject, Filter, Rejection, Reply,
};

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

#[derive(Debug)]
struct DuplicateResource;
impl reject::Reject for DuplicateResource {}

#[derive(Debug)]
struct DuplicateResourceWithData {
    context: Option<Context>,
    user: Option<models::user::User>,
    page: Option<models::page::Page>,
}
impl reject::Reject for DuplicateResourceWithData {}

#[derive(Debug)]
struct NotFound;
impl reject::Reject for NotFound {}

#[derive(Debug)]
struct NotAuthorized;
impl reject::Reject for NotAuthorized {}

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
    if let Some(_) = err.find::<DuplicateResource>() {
        error_reply_body(StatusCode::BAD_REQUEST, String::from("DUPLICATE"))
    } else if let Some(_) = err.find::<NotAuthorized>() {
        let html = String::from("You are not authorized to do this");
        error_reply_body(StatusCode::FORBIDDEN, html)
    } else if let Some(_) = err.find::<reject::MissingCookie>() {
        let html = String::from("Try logging in");
        error_reply_body(StatusCode::FORBIDDEN, html)
    } else if let Some(NotFound) = err.find::<NotFound>() {
        error_reply_body(StatusCode::NOT_FOUND, String::from("NOT_FOUND"))
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let message = match e.source() {
            Some(cause) => format!("BAD_REQUEST: {cause}"),
            None => String::from("BAD_REQUEST"),
        };
        error_reply_body(StatusCode::BAD_REQUEST, message)
    } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
        error_reply_body(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            String::from("UNSUPPORTED_MEDIA_TYPE"),
        )
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        log::info!("Passing MethodNotAllowed error through!");
        Err(err)
    } else if err.is_not_found() {
        log::info!("Passing 404 error through!");
        Err(err)
    } else {
        // We should have expected this... Just log and say its a 500
        error!("unhandled rejection: {:?}", err);
        error_reply_body(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("UNHANDLED_REJECTION"),
        )
    }
}

pub fn error_reply_body(code: StatusCode, message: String) -> Result<impl Reply, Rejection> {
    log::error!("{}, {}", code, message);

    let html = warp::reply::html(views::error::error(code, &message));
    Ok(warp::reply::with_status(html, code))
}

pub fn error_reply(code: StatusCode, message: String) -> Result<impl Reply, Rejection> {
    log::error!("{}, {}", code, message);

    let html = warp::reply::html(message);
    Ok(warp::reply::with_status(html, code))
}
