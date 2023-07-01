macro_rules! user {
    () => {
        routes::user::get_by_id()
            .and_then(handlers::user::profile)
            .or(routes::user::login().and_then(handlers::user::profile_with_cookie))
            .or(routes::user::signup().and_then(handlers::user::profile_with_cookie))
            .or(routes::user::signup_form().and_then(handlers::user::signup_form))
            .or(routes::user::login_form().and_then(handlers::user::login_form))
            .recover(handle_rejection)
            .with(warp::trace::named("user"))
    };
}

pub(crate) use user;
