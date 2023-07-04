use crate::models;
use std::include_str;

pub fn link(link: &models::link::Link, page_link: &models::page_link::PageLink) -> String {
    page_link.inject_values(&link.inject_values(include_str!("link.html")))
}

pub fn link_authenticated(link: &models::link::Link, page_link: &models::page_link::PageLink) -> String {
    page_link.inject_values(&link.inject_values(include_str!("link-authenticated.html")))
}
