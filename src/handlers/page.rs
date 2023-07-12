use crate::{error_reply, models, server::Context, views, DuplicateResourceWithData};
use hyper::StatusCode;
use std::convert::Infallible;

pub async fn view(
    context: Context,
    user: models::user::User,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &expanded_page.page)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .enumerate()
        .map(|(i, (link, page_link))| {
            expanded_page
                .page
                .inject_values(&views::link::link(i, link, page_link))
        })
        .collect::<String>();

    let page_html = views::page::view(user, expanded_page, "").replace("{links}", &links);

    Ok(warp::reply::html(page_html))
}

pub async fn view_authenticated(
    context: Context,
    user: models::user::User,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &expanded_page.page)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .enumerate()
        .map(|(i, (link, page_link))| {
            expanded_page
                .page
                .inject_values(&views::link::link_authenticated(i, link, page_link))
        })
        .collect::<String>();

    let page_html =
        views::page::view_authenticated(user, expanded_page, "").replace("{links}", &links);

    Ok(warp::reply::html(page_html))
}

pub async fn create_page(
    _context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::page::create_page(
        expanded_user.user,
        expanded_user.background,
        "",
    )))
}

pub async fn handle_create_link_error(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(resource) = err.find::<DuplicateResourceWithData>() {
        if let Some(expanded_user) = resource.expanded_user.clone() && let Some(expanded_page) = resource.expanded_page.clone() && let Some(context) = resource.context.clone() {
            let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &expanded_page.page)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .enumerate()
        .map(|(i, (link, page_link))| expanded_page.page.inject_values(&views::link::link_authenticated(i, link, page_link)))
        .collect::<String>();

    let html = views::page::view_authenticated(expanded_user.user, expanded_page, "Error: Link already exists in this page").replace("{links}", &links);
            error_reply(StatusCode::CONFLICT, html)
        } else {
            error_reply(StatusCode::INTERNAL_SERVER_ERROR, views::error::error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
        }
    } else {
        Err(err)
    }
}

pub async fn handle_create_page_error(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(resource) = err.find::<DuplicateResourceWithData>() {
        if let Some(expanded_user) = resource.expanded_user.clone() {
            let html = views::page::create_page(
                expanded_user.user,
                expanded_user.background,
                "Error: Page already exists with this name",
            );
            error_reply(StatusCode::CONFLICT, html)
        } else {
            error_reply(
                StatusCode::INTERNAL_SERVER_ERROR,
                views::error::error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
            )
        }
    } else {
        Err(err)
    }
}
