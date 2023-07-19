use crate::{models, views};
use std::include_str;

pub fn profile(
    user: models::user::User,
    background: models::background::Background,
    pages: Vec<models::page::Page>,
    message: &str,
) -> String {
    let pages_html = pages_authenticated(pages);
    views::body::document_authenticated(
        String::from("Profile"),
        &user,
        user.inject_values(include_str!("profile.html"))
            .replace("{pages}", &pages_html)
            .replace("{error}", message)
            .replace("{background}", &background.to_call()),
    )
}

pub fn login_form(message: &str) -> String {
    views::body::document(
        String::from("Login"),
        String::from(include_str!("login.html"))
            .replace("{error}", message)
            .replace("{background}", &models::background::login()),
    )
}

pub fn signup_form(message: &str) -> String {
    views::body::document(
        String::from("Signup"),
        String::from(include_str!("signup.html"))
            .replace("{error}", message)
            .replace("{background}", &models::background::signup()),
    )
}

pub fn pages_authenticated(pages: Vec<models::page::Page>) -> String {
    if pages.len() != 0 {
        pages
            .iter()
            .map(|page| views::page::list_item_authenticated(page))
            .collect::<String>()
    } else {
        String::from(
            "<div class='neubrutalist-card'><h5 class='empty-error'>You have no groups yet! Add one using the form above.</h5></div>",
        )
    }
}
