use crate::{models, views};
use std::include_str;

pub fn view(
    user: models::user::User,
    expanded_page: models::page::ExpandedPage,
    message: &str,
) -> String {
    views::body::document_authenticated(
        expanded_page.page.name.clone(),
        &user,
        expanded_page
            .page
            .inject_values(include_str!("page.html"))
            .replace("{error}", message)
            .replace("{background}", &expanded_page.background.to_call()),
    )
}

pub fn view_authenticated(
    user: models::user::User,
    expanded_page: models::page::ExpandedPage,
    message: &str,
) -> String {
    views::body::document_authenticated(
        expanded_page.page.name.clone(),
        &user,
        user.inject_values(
            &expanded_page
                .page
                .inject_values(include_str!("page-authenticated.html")),
        )
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
