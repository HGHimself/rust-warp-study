macro_rules! hello {
    () => {
        routes::hello::hello_post()
            .and_then(handlers::hello::hello_post)
            .or(routes::hello::hello()
                .and_then(handlers::hello::hello)
                .recover(handlers::hello::hello_rejection))
            .or(routes::hello::sleepy().and_then(handlers::hello::sleepy))
    };
}

pub(crate) use hello;
