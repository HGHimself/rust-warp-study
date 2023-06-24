use crate::{models};
use std::include_str;

pub fn link(link: &models::link::Link) -> String {
    link.inject_values(include_str!("link.html"))
}
