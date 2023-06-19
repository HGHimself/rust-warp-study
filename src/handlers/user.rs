use crate::{models::user, server::Context, views};
use std::convert::Infallible;

pub async fn profile(context: Context, user: user::User) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::user::profile(user)))
}

pub async fn create_user() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::user::create_user()))
}
