use crate::{models, server::Context, utils, NotFound};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

use super::user::authenticate_cookie;

fn path_prefix() -> BoxedFilter<()> {
    warp::path("link").boxed()
}

pub fn get_by_id() -> BoxedFilter<(
    Context,
    models::user::User,
    models::link::Link,
    Vec<models::page::Page>,
)> {
    path_prefix()
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(authenticate_cookie())
        .and_then(with_link)
        .untuple_one()
        .and_then(with_pages_containing_link)
        .untuple_one()
        .boxed()
}

pub fn get_by_id_unauthenticated(
) -> BoxedFilter<(Context, models::link::Link, Vec<models::page::Page>)> {
    path_prefix()
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(filters::ext::get::<Context>())
        .and_then(with_link_unauthenticated)
        .untuple_one()
        .and_then(with_pages_containing_link_unauthenticated)
        .untuple_one()
        .boxed()
}

pub fn get_random_link() -> BoxedFilter<(Context, models::link::Link, Vec<models::page::Page>)> {
    path_prefix()
        .and(warp::path("random"))
        .and(warp::get())
        .and(warp::path::end())
        .and(filters::ext::get::<Context>())
        .and_then(with_random_link)
        .untuple_one()
        .and_then(with_link_unauthenticated)
        .untuple_one()
        .and_then(with_pages_containing_link_unauthenticated)
        .untuple_one()
        .boxed()
}

async fn with_random_link(context: Context) -> Result<(i32, Context), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    let max_id = models::link::get_count_of_links(&mut conn).map_err(|e| {
        log::error!("{:?}", e);
        warp::reject()
    })?;

    Ok((utils::random(max_id, 0) as i32, context))
}

async fn with_link(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<(Context, models::user::User, models::link::Link), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for link with id of {}", id);
    let link = models::link::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, expanded_user.user, link))
}

async fn with_link_unauthenticated(
    id: i32,
    context: Context,
) -> Result<(Context, models::link::Link), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for link with id of {}", id);
    let link = models::link::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, link))
}

async fn with_pages_containing_link(
    context: Context,
    user: models::user::User,
    link: models::link::Link,
) -> Result<
    (
        Context,
        models::user::User,
        models::link::Link,
        Vec<models::page::Page>,
    ),
    warp::Rejection,
> {
    let mut conn = context.db_conn.get_conn();

    let pages =
        models::page::read_pages_by_link(&mut conn, &link).map_err(|_| reject::custom(NotFound))?;

    Ok((context, user, link, pages))
}

async fn with_pages_containing_link_unauthenticated(
    context: Context,
    link: models::link::Link,
) -> Result<(Context, models::link::Link, Vec<models::page::Page>), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let pages =
        models::page::read_pages_by_link(&mut conn, &link).map_err(|_| reject::custom(NotFound))?;

    Ok((context, link, pages))
}
