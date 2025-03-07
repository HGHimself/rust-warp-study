macro_rules! page {
    () => {
        warp::path("page")
            .and(
                routes::page::get_authenticated()
                    .and_then(handlers::page::view_authenticated)
                    .or(routes::page::get().and_then(handlers::page::view))
                    .or(routes::page::get_unauthenticated()
                        .and_then(handlers::page::view_unauthenticated))
                    .or(routes::page::create_link()
                        .and_then(handlers::page::view_authenticated)
                        .recover(handlers::page::handle_create_link_error))
                    .or(routes::page::create()
                        .and_then(handlers::page::view_authenticated)
                        .recover(handlers::page::handle_create_page_error))
                    .or(routes::page::delete().and_then(handlers::user::profile))
                    .or(routes::page::delete_link().and_then(handlers::page::view_authenticated))
                    .recover(handle_rejection),
            )
            .with(warp::trace::named("page"))
    };
}

pub(crate) use page;
