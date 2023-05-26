use warp::{filters::BoxedFilter, fs::File, Filter};

pub fn get_static() -> BoxedFilter<(File,)> {
    warp::path("static")
        .and(warp::fs::dir("./www/static"))
        .boxed()
}
