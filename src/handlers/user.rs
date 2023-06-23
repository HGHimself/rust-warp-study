use crate::{models, server::Context, views};
use std::convert::Infallible;

pub async fn profile(
    context: Context,
    user: models::user::User,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let pages = models::page::read_pages_by_user_id(&mut conn, user.id)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .map(|page| views::page::list_item(page))
        .collect::<String>();
    let profile_html = views::user::profile(user).replace("{pages}", &pages);

    Ok(warp::reply::html(profile_html))
}

pub async fn create_user() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::user::create_user()))
}
