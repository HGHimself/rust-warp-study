use crate::{models, server::Context, NotFound};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("link").boxed()
}

pub fn get_by_id() -> BoxedFilter<(Context, models::link::Link, Vec<models::page::Page>)> {
    path_prefix()
        .and(warp::get())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_link)
        .untuple_one()
        .and_then(with_pages_containing_link)
        .untuple_one()
        .boxed()
}

async fn with_link(
    context: Context,
    id: i32,
) -> Result<(Context, models::link::Link), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for link with id of {}", id);
    let link = models::link::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, link))
}

async fn with_pages_containing_link(
    context: Context,
    link: models::link::Link,
) -> Result<(Context, models::link::Link, Vec<models::page::Page>), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let pages =
        models::page::read_pages_by_link(&mut conn, &link).map_err(|_| reject::custom(NotFound))?;

    Ok((context, link, pages))
}
