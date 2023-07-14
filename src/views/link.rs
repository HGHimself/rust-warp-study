use crate::models;
use std::include_str;

pub fn link(
    _i: usize,
    link: &models::link::Link,
    page_link: &models::page_link::PageLink,
) -> String {
    page_link.inject_values(&link.inject_values(include_str!("link.html")))
    // .replace("{y}", &(i + 1).to_string())
    // .replace("{x}", &random(5, 1).to_string())
}

pub fn link_authenticated(
    _i: usize,
    link: &models::link::Link,
    page_link: &models::page_link::PageLink,
) -> String {
    page_link.inject_values(&link.inject_values(include_str!("link-authenticated.html")))
    // .replace("{y}", &(i + 1).to_string())
    // .replace("{x}", &random(5, 1).to_string())
}
