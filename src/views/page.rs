use crate::{
    models::{self},
    views,
};
use std::include_str;

pub fn view(
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    message: &str,
) -> String {
    let links_html = views::link::links_to_list(links, &expanded_page);

    views::body::document_authenticated(
        expanded_page.page.name.clone(),
        &expanded_user.user,
        expanded_page
            .page
            .inject_values(include_str!("page.html"))
            .replace("{links}", &links_html)
            .replace("{error}", message)
            .replace("{background}", &expanded_page.background.to_call()),
    )
}

pub fn view_unauthenticated(
    expanded_page: models::page::ExpandedPage,
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    message: &str,
) -> String {
    let links_html = views::link::links_to_list(links, &expanded_page);

    views::body::document(
        expanded_page.page.name.clone(),
        expanded_page
            .page
            .inject_values(include_str!("page.html"))
            .replace("{links}", &links_html)
            .replace("{error}", message)
            .replace("{background}", &expanded_page.background.to_call()),
    )
}

pub fn view_authenticated(
    expanded_user: models::user::ExpandedUser,
    expanded_page: models::page::ExpandedPage,
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    message: &str,
) -> String {
    let links_html = views::link::links_to_list_authenticated(links, &expanded_page);

    views::body::document_authenticated(
        expanded_page.page.name.clone(),
        &expanded_user.user,
        expanded_user
            .user
            .inject_values(
                &expanded_page
                    .page
                    .inject_values(include_str!("page-authenticated.html")),
            )
            .replace("{links}", &links_html)
            .replace("{error}", message)
            .replace("{background}", &expanded_page.background.to_call()),
    )
}

pub fn list_item(page: &models::page::Page) -> String {
    page.inject_values(include_str!("page-list-item.html"))
}

pub fn list_item_authenticated(page: &models::page::Page) -> String {
    page.inject_values(include_str!("page-list-item-authenticated.html"))
}

pub fn option_item(page: &models::page::Page) -> String {
    page.inject_values(include_str!("page-option-item.html"))
}
