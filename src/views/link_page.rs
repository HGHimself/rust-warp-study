use crate::{models, views};
use std::include_str;

pub fn link_page(link: &models::link::Link, pages: &str) -> String {
    views::body::document(
        link.inject_values(include_str!("link-page.html"))
            .replace("{pages}", pages),
    )
}
