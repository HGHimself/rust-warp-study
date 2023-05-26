use crate::Webauthn;
use log::info;
use std::{convert::Infallible, sync::Arc, time::Duration};
use warp::{http::StatusCode, Rejection};

pub async fn register(
    webauthn: Arc<Webauthn>,
    user_name: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received register request for name");

    let (ccr, skr) = webauthn
        .register(&user_name, &user_name)
        .map_err(|_| warp::reject())?;
    log::debug!("{:?}", skr);
    Ok(warp::reply::json(&ccr))
}
