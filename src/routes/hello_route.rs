use warp::{filters::BoxedFilter, Filter};

// 1. "hello"
fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn hello() -> BoxedFilter<(String,)> {
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .and(warp::path::param::<String>()) // 5.
        .boxed()
}
