use crate::{models, server::Context, DuplicateResource, NotFound};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

use super::user::authenticate_cookie;

fn path_prefix() -> BoxedFilter<()> {
    warp::path("page").boxed()
}

pub fn get_by_id() -> BoxedFilter<(Context, models::page::Page)> {
    path_prefix()
        .and(warp::get())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

async fn with_page(
    context: Context,
    id: i32,
) -> Result<(Context, models::page::Page), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, page))
}

pub fn create() -> BoxedFilter<(Context, models::page::Page)> {
    path_prefix()
        .and(warp::post())
        .and(warp::path::end())
        .and(authenticate_cookie())
        .and(warp::body::form::<models::page::NewPageApi>())
        .and_then(insert_new_page)
        .untuple_one()
        .boxed()
}

async fn insert_new_page(
    context: Context,
    user: models::user::User,
    _session: models::session::Session,
    new_page: models::page::NewPageApi,
) -> Result<(Context, models::page::Page), warp::Rejection> {
    log::info!("Saving Page");
    let mut conn = context.db_conn.get_conn();
    let page = models::page::NewPage::new(new_page, user.id)
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    reject::custom(DuplicateResource)
                }
                _ => reject::reject(),
            }
        })?;
    log::info!("Saved Page");
    Ok((context, page))
}

pub fn create_form() -> BoxedFilter<()> {
    path_prefix()
        .and(warp::get())
        .and(warp::path("create"))
        .and(warp::path::end())
        .boxed()
}

pub fn create_link() -> BoxedFilter<(Context, models::page::Page)> {
    path_prefix()
        .and(warp::post())
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(warp::path::end())
        .and(authenticate_cookie())
        .and(warp::body::form::<models::link::NewLinkApi>())
        .and_then(with_new_link)
        .untuple_one()
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

async fn with_new_link(
    page_id: i32,
    context: Context,
    user: models::user::User,
    _session: models::session::Session,
    new_link: models::link::NewLinkApi,
) -> Result<(Context, i32), warp::Rejection> {
    log::info!("Saving Link");
    let mut conn = context.db_conn.get_conn();
    let name = new_link.name.clone();

    let link = match models::link::read_by_url(&mut conn, new_link.url.clone()) {
        Err(diesel::NotFound) => models::link::NewLink::new(new_link, user.id)
            .insert(&mut conn)
            .map_err(|_| reject::reject()),
        Ok(link) => Ok(link),
        _ => Err(warp::reject()),
    }?;

    models::page_link::NewPageLink::new(page_id, link.id, name)
        .insert(&mut conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                reject::custom(DuplicateResource)
            }
            _ => reject::reject(),
        })?;

    log::info!("Saved Link");
    Ok((context, page_id))
}

pub fn remove_link() -> BoxedFilter<(Context, models::page::Page)> {
    path_prefix()
        .and(warp::delete())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_remove_link)
        .untuple_one()
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

async fn with_remove_link(
    context: Context,
    page_id: i32,
    link_id: i32,
) -> Result<(Context, i32), warp::Rejection> {
    log::info!("Removing PageLink");
    let mut conn = context.db_conn.get_conn();
    models::page_link::remove_link_by_page_id_and_link_id(&mut conn, page_id, link_id)
        .map_err(|_| reject::custom(NotFound))?;

    log::info!("Removed PageLink");
    Ok((context, page_id))
}
