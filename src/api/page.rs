macro_rules! page {
    () => {
        routes::page::get_by_id()
            .and_then(handlers::page::view)
            .or(routes::page::create_form().and_then(handlers::page::create_page))
            .or(routes::page::create_link()
                .and_then(handlers::page::view)
                .recover(handlers::page::handle_create_link_error))
            .or(routes::page::create()
                .and_then(handlers::page::view)
                .recover(handlers::page::handle_create_page_error))
            .or(routes::page::remove_link().and_then(handlers::page::view))
            .recover(handle_rejection)
            .with(warp::trace::named("page"))
    };
}

pub(crate) use page;
