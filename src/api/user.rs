macro_rules! user {
    () => {
        warp::path("user")
            .and(
                routes::user::get_by_cookie()
                    .and_then(handlers::user::profile)
                    .or(routes::user::login()
                        .and_then(handlers::user::profile_with_cookie)
                        .recover(handlers::user::handle_login_errors))
                    .or(routes::user::signup()
                        .and_then(handlers::user::profile_with_cookie)
                        .recover(handlers::user::handle_signup_errors))
                    .or(routes::user::logout()
                        .and_then(handlers::user::logout)
                        .recover(handlers::user::handle_logout_errors))
                    .or(routes::user::signup_form()
                        .and(routes::user::authenticate_cookie())
                        .and_then(handlers::user::profile))
                    .or(routes::user::signup_form().and_then(handlers::user::signup_form))
                    .or(routes::user::login_form()
                        .and(routes::user::authenticate_cookie())
                        .and_then(handlers::user::profile))
                    .or(routes::user::login_form().and_then(handlers::user::login_form))
                    .recover(handle_rejection),
            )
            .with(warp::trace::named("user"))
    };
}

pub(crate) use user;
