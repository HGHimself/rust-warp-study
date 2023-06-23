macro_rules! user {
    () => {
        routes::user::get_by_id()
            .and_then(handlers::user::profile)
            .or(routes::user::create().and_then(handlers::user::profile))
            .or(routes::user::create_form().and_then(handlers::user::create_user))
            .recover(handle_rejection)
            .with(warp::trace::named("user"))
    };
}

pub(crate) use user;
