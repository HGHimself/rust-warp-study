use crate::{models, views};
use std::include_str;

pub fn profile(user: models::user::User, background: models::background::Background) -> String {
    views::body::document_authenticated(
        String::from("Profile"),
        &user,
        user.inject_values(include_str!("profile.html"))
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
