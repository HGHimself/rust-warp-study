use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn sleepy() -> BoxedFilter<(u64,)> {
    warp::get()
        .and(warp::path("sleepy"))
        .and(warp::path::param::<u64>())
        .boxed()
}
