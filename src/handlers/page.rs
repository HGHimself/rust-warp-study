use crate::{error_reply, handlers, models, server::Context, views, DuplicateResourceWithData};
use hyper::StatusCode;

pub async fn view(
    context: Context,
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &expanded_page.page).map_err(|e| {
        log::error!("{:?}", e);
        warp::reject::not_found()
    })?;

    let links_html = links_to_list(links, &expanded_page);

    let page_html =
        views::page::view(expanded_user.user, expanded_page, "").replace("{links}", &links_html);

    Ok(warp::reply::html(page_html))
}

pub async fn view_authenticated(
    context: Context,
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &expanded_page.page).map_err(|e| {
        log::error!("{:?}", e);
        warp::reject::not_found()
    })?;

    let links_html = links_to_list_authenticated(links, &expanded_page);

    let page_html = views::page::view_authenticated(expanded_user.user, expanded_page, "")
        .replace("{links}", &links_html);

    Ok(warp::reply::html(page_html))
}

fn links_to_list(
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    expanded_page: &models::page::ExpandedPage,
) -> String {
    if links.len() != 0 {
        links
            .iter()
            .enumerate()
            .map(|(i, (link, page_link))| {
                expanded_page
                    .page
                    .inject_values(&views::link::link(i, link, page_link))
            })
            .collect::<String>()
    } else {
        String::from("<div class='neubrutalist-card'><h5 class='empty-error'>This page does not have any links, yet.</h5></div>")
    }
}

fn links_to_list_authenticated(
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    expanded_page: &models::page::ExpandedPage,
) -> String {
    if links.len() != 0 {
        links
            .iter()
            .enumerate()
            .map(|(i, (link, page_link))| {
                expanded_page
                    .page
                    .inject_values(&views::link::link_authenticated(i, link, page_link))
            })
            .collect::<String>()
    } else {
        String::from(
            "<div class='neubrutalist-card'><h5 class='empty-error'>You have no links yet! Add one using the form above.</h5></div>",
        )
    }
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
        })?;

    let links_html = links_to_list_authenticated(links, &expanded_page);

    let html = views::page::view_authenticated(expanded_user.user, expanded_page, "Error: Link already exists in this page").replace("{links}", &links_html);
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
        if let Some(expanded_user) = resource.expanded_user.clone() && let Some(context) = resource.context.clone() {
            let mut conn = context.db_conn.get_conn();

            let pages =
            models::page::read_pages_by_user_id(&mut conn, expanded_user.user.id).map_err(|e| {
                log::error!("{:?}", e);
                warp::reject::not_found()
            })?;

            let pages_html = handlers::user::pages_authenticated(pages);

            let html = views::user::profile(
                expanded_user.user,
                expanded_user.background,
                pages_html,
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
