use warp::{filters::BoxedFilter, Filter};

pub fn index() -> BoxedFilter<()> {
    warp::get()
        .and(warp::path::end())
        .or(warp::get().and(warp::path::path("index.html")))
        .unify()
        .boxed()
}
