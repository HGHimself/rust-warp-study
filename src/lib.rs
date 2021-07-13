pub mod handlers;
pub mod routes;
pub mod services;
pub mod telemetry;

use serde_derive::{Deserialize, Serialize};
use warp::reject;

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

#[derive(Debug)]
pub struct NotEven;

impl reject::Reject for NotEven {}
