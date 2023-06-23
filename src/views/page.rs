use crate::{models, views};
use std::include_str;

pub fn view(page: models::page::Page) -> String {
    views::body::document(page.inject_values(include_str!("page.html")))
}

pub fn create_page() -> String {
    views::body::document(String::from(include_str!("create-page.html")))
}

pub fn list_item(page: &models::page::Page) -> String {
    page.inject_values(include_str!("page-list-item.html"))
}
