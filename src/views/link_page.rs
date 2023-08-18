use crate::{models, views};
use std::include_str;

pub fn link_page(
    link: &models::link::Link,
    user: &models::user::User,
    pages: &str,
    add_to_my_page_form: &str,
) -> String {
    views::body::document_authenticated(
        String::from("Link View"),
        user,
        user.inject_values(&link.inject_values(include_str!("link-page-authenticated.html")))
            .replace("{pages}", pages)
            .replace("{add-to-my-page-form}", add_to_my_page_form)
            .replace("{background}", &models::background::background_random()),
    )
}

pub fn link_page_unauthenticated(link: &models::link::Link, pages: &str) -> String {
    views::body::document(
        String::from("Link View"),
        link.inject_values(include_str!("link-page.html"))
            .replace("{pages}", pages)
            .replace("{background}", &models::background::background_random()),
    )
}

pub fn add_to_my_page(link: &models::link::Link, options: String) -> String {
    link.inject_values(include_str!("add-to-my-page.html"))
        .replace("{options}", &options)
}
