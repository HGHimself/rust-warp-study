use crate::{
    error_reply, handlers, models, server::Context, views, ResourceError, ResourceErrorData,
};
use hyper::StatusCode;

pub async fn view(
    context: Context,
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let links = get_links(context, &expanded_page)?;

    let page_html = views::page::view(expanded_user, expanded_page, links, "");

    Ok(warp::reply::html(page_html))
}

pub async fn view_authenticated(
    context: Context,
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let links = get_links(context, &expanded_page)?;

    let page_html = views::page::view_authenticated(expanded_user, expanded_page, links, "");

    Ok(warp::reply::html(page_html))
}

pub async fn view_unauthenticated(
    context: Context,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let links = get_links(context, &expanded_page)?;

    let page_html = views::page::view_unauthenticated(expanded_page, links, "");

    Ok(warp::reply::html(page_html))
}

pub async fn handle_create_link_error(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(ResourceError::Duplicate(resource)) = err.find::<ResourceError>() {
        process_page_error(resource, "Error: Link already exists in this group")
    } else if let Some(ResourceError::TooMany(resource)) = err.find::<ResourceError>() {
        process_page_error(resource, "Error: You cannot add any more links")
    } else {
        Err(err)
    }
}

pub async fn handle_create_page_error(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(ResourceError::Duplicate(resource)) = err.find::<ResourceError>() {
        process_profile_error(resource, "Error: Group with this name already exists")
    } else if let Some(ResourceError::TooMany(resource)) = err.find::<ResourceError>() {
        process_profile_error(resource, "Error: You cannot add any more groups")
    } else {
        Err(err)
    }
}

pub fn process_page_error(
    resource: &ResourceErrorData,
    message: &str,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(expanded_user) = resource.expanded_user.clone() && let Some(expanded_page) = resource.expanded_page.clone() && let Some(context) = resource.context.clone() {

        let links = get_links(context, &expanded_page)?;

        let html = views::page::view_authenticated(
            expanded_user,
            expanded_page,
            links,
            message
        );

        error_reply(StatusCode::CONFLICT, html)
    } else {
        error_reply(StatusCode::INTERNAL_SERVER_ERROR, views::error::error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"))
    }
}

pub fn process_profile_error(
    resource: &ResourceErrorData,
    message: &str,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(expanded_user) = resource.expanded_user.clone() && let Some(context) = resource.context.clone() {
        let pages = handlers::user::get_pages(context, &expanded_user)?;
        let html = views::user::profile(
            expanded_user.user,
            expanded_user.background,
            pages,
            message,
        );
        error_reply(StatusCode::CONFLICT, html)
    } else {
        error_reply(
            StatusCode::INTERNAL_SERVER_ERROR,
            views::error::error(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        )
    }
}

fn get_links(
    context: Context,
    expanded_page: &models::page::ExpandedPage,
) -> Result<Vec<(models::link::Link, models::page_link::PageLink)>, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    models::link::read_links_by_page(&mut conn, &expanded_page.page).map_err(|e| {
        log::error!("{:?}", e);
        warp::reject::not_found()
    })
}
