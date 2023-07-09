use crate::{models, views};
use std::include_str;

pub fn view(user: models::user::User, page: models::page::Page, message: &str) -> String {
    views::body::document_authenticated(
        page.name.clone(),
        &user,
        page.inject_values(include_str!("page.html"))
            .replace("{error}", message)
            .replace("{background}", &models::background::index()),
    )
}

pub fn view_authenticated(
    user: models::user::User,
    page: models::page::Page,
    message: &str,
) -> String {
    views::body::document_authenticated(
        page.name.clone(),
        &user,
        user.inject_values(&page.inject_values(include_str!("page-authenticated.html")))
            .replace("{error}", message)
            .replace("{background}", &models::background::index()),
    )
}

pub fn create_page(user: models::user::User, message: &str) -> String {
    views::body::document_authenticated(
        String::from("Create Page"),
        &user,
        String::from(include_str!("create-page.html"))
            .replace("{error}", message)
            .replace("{background}", &models::background::index()),
    )
}

pub fn list_item(page: &models::page::Page) -> String {
    page.inject_values(include_str!("page-list-item.html"))
}
