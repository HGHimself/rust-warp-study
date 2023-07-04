use crate::{models, views};
use std::include_str;

pub fn profile(user: models::user::User) -> String {
    views::body::document_authenticated(
        String::from("Profile"),
        &user,
        user.inject_values(include_str!("profile.html")),
    )
}

pub fn login_form(message: &str) -> String {
    views::body::document(
        String::from("Login"),
        String::from(include_str!("login.html")).replace("{error}", message),
    )
}

pub fn signup_form(message: &str) -> String {
    views::body::document(
        String::from("Signup"),
        String::from(include_str!("signup.html")).replace("{error}", message),
    )
}
