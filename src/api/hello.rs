macro_rules! hello {
    () => {
        routes::hello::sleepy()
            .and_then(handlers::hello::sleepy)
            .recover(handle_rejection)
            .with(warp::trace::named("hello"))
    };
}

pub(crate) use hello;
