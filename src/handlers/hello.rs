use std::{convert::Infallible, sync::Arc, time::Duration};

// pub async fn hello_rejection(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
//     let code;
//     let message;

//     if let Some(NotEven) = err.find() {
//         message = String::from("NOT_EVEN");
//         code = StatusCode::BAD_REQUEST;

//         let json = warp::reply::json(&ErrorMessage {
//             code: code.as_u16(),
//             message: message,
//         });

//         Ok(warp::reply::with_status(json, code))
//     } else {
//         Err(err)
//     }
// }

pub async fn sleepy(seconds: u64) -> Result<impl warp::Reply, Infallible> {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    Ok(format!("I waited {} seconds!", seconds))
}
