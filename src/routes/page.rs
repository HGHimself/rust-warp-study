use crate::{models, routes, server::Context, DuplicateResourceWithData, NotFound};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use warp::{filters::BoxedFilter, reject, Filter};

use super::user::authenticate_cookie;

fn path_prefix() -> BoxedFilter<()> {
    warp::path("page").boxed()
}

pub fn get() -> BoxedFilter<(Context, models::user::User, models::page::Page)> {
    path_prefix()
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::user::authenticate_cookie())
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

pub fn get_authenticated() -> BoxedFilter<(Context, models::user::User, models::page::Page)> {
    path_prefix()
        .and(warp::get())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::user::authenticate_cookie())
        .and_then(with_authenticated_page)
        .untuple_one()
        .boxed()
}

async fn with_authenticated_page(
    id: i32,
    context: Context,
    user: models::user::User,
    _session: models::session::Session,
) -> Result<(Context, models::user::User, models::page::Page), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id_and_user_id(&mut conn, id, user.id)
        .map_err(|_| reject::custom(NotFound))?;
    Ok((context, user, page))
}

async fn with_page(
    id: i32,
    context: Context,
    user: models::user::User,
    _session: models::session::Session,
) -> Result<(Context, models::user::User, models::page::Page), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, user, page))
}

pub fn create() -> BoxedFilter<(Context, models::user::User, models::page::Page)> {
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
) -> Result<(Context, models::user::User, models::page::Page), warp::Rejection> {
    log::info!("Saving Page");
    let mut conn = context.db_conn.get_conn();
    let page = models::page::NewPage::new(new_page, user.id)
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    reject::custom(DuplicateResourceWithData {
                        context: Some(context.clone()),
                        user: Some(user.clone()),
                        page: None,
                    })
                }
                _ => reject::reject(),
            }
        })?;
    log::info!("Saved Page");
    Ok((context, user, page))
}

pub fn create_form() -> BoxedFilter<(Context, models::user::User, models::session::Session)> {
    path_prefix()
        .and(warp::get())
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(routes::user::authenticate_cookie())
        .boxed()
}

pub fn create_link() -> BoxedFilter<(Context, models::user::User, models::page::Page)> {
    path_prefix()
        .and(warp::post())
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(warp::path::end())
        .and(authenticate_cookie())
        .and_then(with_authenticated_page)
        .untuple_one()
        .and(warp::body::form::<models::link::NewLinkApi>())
        .and_then(with_new_link)
        .untuple_one()
        .boxed()
}

async fn with_new_link(
    context: Context,
    user: models::user::User,
    page: models::page::Page,
    new_link: models::link::NewLinkApi,
) -> Result<(Context, models::user::User, models::page::Page), warp::Rejection> {
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

    models::page_link::NewPageLink::new(page.id, link.id, name)
        .insert(&mut conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                reject::custom(DuplicateResourceWithData {
                    context: Some(context.clone()),
                    user: Some(user.clone()),
                    page: Some(page.clone()),
                })
            }
            _ => reject::reject(),
        })?;

    log::info!("Saved Link");
    Ok((context, user, page))
}

pub fn remove_link() -> BoxedFilter<(Context, models::user::User, models::page::Page)> {
    path_prefix()
        .and(warp::delete())
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(authenticate_cookie())
        .and_then(with_authenticated_page)
        .untuple_one()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_remove_link)
        .untuple_one()
        .boxed()
}

async fn with_remove_link(
    context: Context,
    user: models::user::User,
    page: models::page::Page,
    link_id: i32,
) -> Result<(Context, models::user::User, models::page::Page), warp::Rejection> {
    log::info!("Removing PageLink");
    let mut conn = context.db_conn.get_conn();
    models::page_link::remove_link_by_page_id_and_link_id(&mut conn, page.id, link_id)
        .map_err(|_| reject::custom(NotFound))?;

    log::info!("Removed PageLink");
    Ok((context, user, page))
}
