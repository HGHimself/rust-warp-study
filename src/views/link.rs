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

pub fn links_to_list_authenticated(
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    expanded_page: &models::page::ExpandedPage,
) -> String {
    if links.len() != 0 {
        links
            .iter()
            .enumerate()
            .map(|(i, (link, page_link))| {
                expanded_page
                    .page
                    .inject_values(&link_authenticated(i, link, page_link))
            })
            .collect::<String>()
    } else {
        String::from(
            "<div class='neubrutalist-card'><h5 class='empty-error'>You have no links yet! Add one using the form above.</h5></div>",
        )
    }
}

pub fn links_to_list(
    links: Vec<(models::link::Link, models::page_link::PageLink)>,
    expanded_page: &models::page::ExpandedPage,
) -> String {
    if links.len() != 0 {
        links
            .iter()
            .enumerate()
            .map(|(i, (da_link, page_link))| {
                expanded_page
                    .page
                    .inject_values(&link(i, &da_link, page_link))
            })
            .collect::<String>()
    } else {
        String::from("<div class='neubrutalist-card'><h5 class='empty-error'>This page does not have any links, yet.</h5></div>")
    }
}
