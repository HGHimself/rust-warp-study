macro_rules! link_api {
    () => {
        routes::link::get_by_id()
            .and_then(handlers::link::link_pages)
            .or(routes::link::get_by_id_unauthenticated()
                .and_then(handlers::link::link_pages_unauthenticated))
            .or(routes::link::get_random_link_authenticated().and_then(handlers::link::link_pages))
            .or(routes::link::get_random_link()
                .and_then(handlers::link::link_pages_unauthenticated))
            .or(routes::link::add_link_to_my_page().and_then(handlers::page::view_authenticated))
            .recover(handle_rejection)
            .with(warp::trace::named("link"))
    };
}

pub(crate) use link_api;
