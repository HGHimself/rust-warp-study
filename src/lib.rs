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
extern crate opengraph;

use server::Context;
use std::convert::Infallible;
use std::error::Error;
use warp::{http::StatusCode, reject, Rejection, Reply};

pub const MAX_PAGE_COUNT: usize = 10;
pub const MAX_LINK_COUNT: usize = 20;

#[derive(Debug)]
pub enum ResourceError {
    TooMany(ResourceErrorData),
    Duplicate(ResourceErrorData),
}

#[derive(Debug)]
pub struct ResourceErrorData {
    context: Option<Context>,
    expanded_user: Option<models::user::ExpandedUser>,
    expanded_page: Option<models::page::ExpandedPage>,
}

impl reject::Reject for ResourceError {}

#[derive(Debug)]
struct NotFound;
impl reject::Reject for NotFound {}

#[derive(Debug)]
struct NotAuthorized;
impl reject::Reject for NotAuthorized {}

#[derive(Debug)]
struct OldCookie;
impl reject::Reject for OldCookie {}

pub async fn handle_final_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        log::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    log::error!("{}, {}", code, message);

    let html = warp::reply::html(views::error::error(code, message));
    Ok(warp::reply::with_status(html, code))
}

pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn warp::Reply>, Rejection> {
    if let Some(ResourceError::Duplicate(_)) = err.find::<ResourceError>() {
        let code = StatusCode::BAD_REQUEST;
        error_reply(code, views::error::error(code, "Duplicate resource"))
    } else if let Some(ResourceError::TooMany(_)) = err.find::<ResourceError>() {
        let code = StatusCode::BAD_REQUEST;
        error_reply(code, views::error::error(code, "Too many resources"))
    } else if let Some(_) = err.find::<NotAuthorized>() {
        let code = StatusCode::FORBIDDEN;
        error_reply(
            code,
            views::error::error(code, "You are not authorized to do this"),
        )
    } else if let Some(_) = err.find::<OldCookie>() {
        Ok(Box::new(warp::reply::with_header(
            warp::reply::html(views::body::index("Your session has expired")),
            "Set-Cookie",
            format!("session=; Path=/"),
        )))
    } else if let Some(_) = err.find::<reject::MissingCookie>() {
        let code = StatusCode::FORBIDDEN;
        error_reply(code, views::error::error(code, "You are not logged in"))
    } else if let Some(NotFound) = err.find::<NotFound>() {
        let code = StatusCode::NOT_FOUND;
        error_reply(
            code,
            views::error::error(code, "We could not locate this resource"),
        )
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let message = match e.source() {
            Some(cause) => cause.to_string(),
            None => String::from("The request was malformed"),
        };
        let code = StatusCode::NOT_FOUND;
        error_reply(code, views::error::error(code, &message))
    } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
        let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        error_reply(code, views::error::error(code, "UNSUPPORTED_MEDIA_TYPE"))
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        log::info!("Passing MethodNotAllowed error through!");
        Err(err)
    } else if err.is_not_found() {
        log::info!("Passing 404 error through!");
        Err(err)
    } else {
        // We should have expected this... Just log and say its a 500
        log::error!("unhandled rejection: {:?}", err);
        error_reply(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("UNHANDLED_REJECTION"),
        )
    }
}

pub fn error_reply(code: StatusCode, message: String) -> Result<Box<dyn Reply>, Rejection> {
    log::error!("{}", code);

    let html = warp::reply::html(message);
    Ok(Box::new(warp::reply::with_status(html, code)))
}
