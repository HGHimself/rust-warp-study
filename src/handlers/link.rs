use crate::{models, server::Context, views};
use std::convert::Infallible;

pub async fn link_pages(
    _context: Context,
    user: models::user::User,
    link: models::link::Link,
    pages: Vec<models::page::Page>,
) -> Result<impl warp::Reply, Infallible> {
    let pages_html = pages_to_list(pages);

    let link_page_html = views::link_page::link_page(&link, &user, &pages_html);

    Ok(warp::reply::html(link_page_html))
}

pub async fn link_pages_unauthenticated(
    _context: Context,
    link: models::link::Link,
    pages: Vec<models::page::Page>,
) -> Result<impl warp::Reply, Infallible> {
    let pages_html = pages_to_list(pages);

    let link_page_html = views::link_page::link_page_unauthenticated(&link, &pages_html);

    Ok(warp::reply::html(link_page_html))
}

fn pages_to_list(pages: Vec<models::page::Page>) -> String {
    if pages.len() != 0 {
        pages
            .iter()
            .map(|page| views::page::list_item(page))
            .collect::<String>()
    } else {
        String::from("<h3>This link has not been saved under any pages, yet!</h3>")
    }
}
