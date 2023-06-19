macro_rules! hello {
    () => {
        routes::hello::sleepy()
            .and_then(handlers::hello::sleepy)
            .with(warp::trace::named("hello"))
    };
}

pub(crate) use hello;
