macro_rules! index_api {
    () => {
        routes::index::index()
            .and(routes::user::authenticate_cookie())
            .and_then(handlers::index::index_authenticated)
            .or(routes::index::index().and_then(handlers::index::index))
    };
}

pub(crate) use index_api;
