use crate::{models, routes, server::Context, DuplicateResourceWithData, NotFound};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use warp::{filters::BoxedFilter, reject, Filter};

pub fn get() -> BoxedFilter<(Context, models::user::User, models::page::ExpandedPage)> {
    warp::path::param::<i32>()
        .and(warp::path::end())
        .and(warp::get())
        .and(routes::user::authenticate_cookie())
        .and_then(with_page)
        .untuple_one()
        .boxed()
}

pub fn get_authenticated() -> BoxedFilter<(Context, models::user::User, models::page::ExpandedPage)>
{
    warp::path::param::<i32>()
        .and(warp::path::end())
        .and(warp::get())
        .and(routes::user::authenticate_cookie())
        .and_then(with_authenticated_page)
        .untuple_one()
        .boxed()
}

async fn with_authenticated_page(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<(Context, models::user::User, models::page::ExpandedPage), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id_and_user_id(&mut conn, id, expanded_user.user.id)
        .map_err(|_| reject::custom(NotFound))?;
    Ok((context, expanded_user.user, page))
}

async fn with_page(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<(Context, models::user::User, models::page::ExpandedPage), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for page with id of {}", id);
    let page = models::page::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, expanded_user.user, page))
}

pub fn create() -> BoxedFilter<(Context, models::user::User, models::page::ExpandedPage)> {
    warp::path::end()
        .and(warp::post())
        .and(routes::user::authenticate_cookie())
        .and(warp::body::form::<models::page::NewPageApi>())
        .and_then(insert_new_page)
        .untuple_one()
        .boxed()
}

async fn insert_new_page(
    context: Context,
    expanded_user: models::user::ExpandedUser,
    new_page: models::page::NewPageApi,
) -> Result<(Context, models::user::User, models::page::ExpandedPage), warp::Rejection> {
    log::info!("Saving Page");
    let mut conn = context.db_conn.get_conn();
    let background = models::background::random_bg()
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject()
        })?;

    let page = models::page::NewPage::new(new_page, expanded_user.user.id, background.id)
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    reject::custom(DuplicateResourceWithData {
                        context: Some(context.clone()),
                        user: None,
                        expanded_user: Some(expanded_user.clone()),
                        expanded_page: None,
                    })
                }
                _ => reject::reject(),
            }
        })?;
    log::info!("Saved Page");
    Ok((
        context,
        expanded_user.user,
        models::page::expand(page, background),
    ))
}

pub fn create_form() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::path("create")
        .and(warp::path::end())
        .and(warp::get())
        .and(routes::user::authenticate_cookie())
        .boxed()
}

pub fn create_link() -> BoxedFilter<(Context, models::user::User, models::page::ExpandedPage)> {
    warp::path::param::<i32>()
        .and(warp::path("link"))
        .and(warp::path::end())
        .and(warp::post())
        .and(routes::user::authenticate_cookie())
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
    expanded_page: models::page::ExpandedPage,
    new_link: models::link::NewLinkApi,
) -> Result<(Context, models::user::User, models::page::ExpandedPage), warp::Rejection> {
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

    models::page_link::NewPageLink::new(expanded_page.page.id, link.id, name)
        .insert(&mut conn)
        .map_err(|e| match e {
            DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                reject::custom(DuplicateResourceWithData {
                    context: Some(context.clone()),
                    user: Some(user.clone()),
                    expanded_user: None,
                    expanded_page: Some(expanded_page.clone()),
                })
            }
            _ => reject::reject(),
        })?;

    log::info!("Saved Link");
    Ok((context, user, expanded_page))
}

pub fn remove_link() -> BoxedFilter<(Context, models::user::User, models::page::ExpandedPage)> {
    warp::delete()
        .and(warp::path::param::<i32>())
        .and(warp::path("link"))
        .and(routes::user::authenticate_cookie())
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
    expanded_page: models::page::ExpandedPage,
    link_id: i32,
) -> Result<(Context, models::user::User, models::page::ExpandedPage), warp::Rejection> {
    log::info!("Removing PageLink");
    let mut conn = context.db_conn.get_conn();
    models::page_link::remove_link_by_page_id_and_link_id(
        &mut conn,
        expanded_page.page.id,
        link_id,
    )
    .map_err(|_| reject::custom(NotFound))?;

    log::info!("Removed PageLink");
    Ok((context, user, expanded_page))
}
