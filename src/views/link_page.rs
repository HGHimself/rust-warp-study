use crate::{models, views};
use std::include_str;

pub fn link_page(link: &models::link::Link, user: &models::user::User, pages: &str) -> String {
    views::body::document_authenticated(
        String::from("Link View"),
        user,
        user.inject_values(&link.inject_values(include_str!("link-page.html")))
            .replace("{pages}", pages)
            .replace("{background}", &models::background::background_random()),
    )
}
