macro_rules! link_api {
    () => {
        routes::link::get_by_id()
            .and_then(handlers::link::link_pages)
            .recover(handle_rejection)
            .with(warp::trace::named("link"))
    };
}

pub(crate) use link_api;
