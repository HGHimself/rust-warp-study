use crate::{models, server::Context, views};
use std::convert::Infallible;

pub async fn view(
    context: Context,
    page: models::page::Page,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let links = models::link::read_links_by_page(&mut conn, &page)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject::not_found()
        })?
        .iter()
        .map(|link| views::link::link(link))
        .collect::<String>();

    let page_html = views::page::view(page).replace("{links}", &links);

    Ok(warp::reply::html(page_html))
}

pub async fn create_page() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::page::create_page()))
}
