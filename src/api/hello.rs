#[macro_export]
macro_rules! hello {
    () => {
        hello_route::hello_post()
            .and_then(hello_handler::hello_post)
            .or(hello_route::hello()
                .and_then(hello_handler::hello)
                .recover(hello_handler::hello_rejection))
    };
}
