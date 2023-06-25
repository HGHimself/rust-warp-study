use crate::views;
use std::include_str;
use warp::http::StatusCode;

pub fn error(error: StatusCode, message: &str) -> String {
    views::body::document(
        String::from(error.as_str()),
        include_str!("error.html")
            .replace("{error}", error.as_str())
            .replace("{message}", message),
    )
}
