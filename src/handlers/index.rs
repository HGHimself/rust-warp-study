use crate::{models, server::Context, views};
use std::convert::Infallible;

pub async fn index() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::body::index("")))
}

pub async fn index_authenticated(
    _context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::body::index_authenticated(
        &expanded_user.user,
        "",
    )))
}
