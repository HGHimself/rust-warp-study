use crate::{models, server::Context, views};
use std::convert::Infallible;

pub async fn link_pages(
    _context: Context,
    user: models::user::User,
    link: models::link::Link,
    pages: Vec<models::page::Page>,
) -> Result<impl warp::Reply, Infallible> {
    let pages_html = pages
        .iter()
        .map(|page| views::page::list_item(page))
        .collect::<String>();
    let link_page_html = views::link_page::link_page(&link, &user, &pages_html);

    Ok(warp::reply::html(link_page_html))
}
