use crate::{models, views};
use std::include_str;

pub fn profile(user: models::user::User) -> String {
    views::body::document(user.inject_values(include_str!("profile.html")))
}

pub fn create_user() -> String {
    views::body::document(String::from(include_str!("create-user.html")))
}

// pub fn edit_user(user: models::user::User) -> String {
//     let first_name = user.first_name;
//     let middle_name = user.middle_name;
//     let last_name = user.last_name;
//     let email = user.email;
//     let birthday = user.birthday;

//     views::body::document(String::from(include_str!("edit-user.html")))
// }
