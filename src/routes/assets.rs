use crate::{is_static, with_subdomain};
use warp::{filters::BoxedFilter, fs::File, Filter};

pub fn get_static() -> BoxedFilter<(File,)> {
    warp::path("static")
        // warp::any()
        // .and(with_subdomain())
        //     .and_then(is_static)
        // .untuple_one()
        .and(warp::fs::dir("./www/static"))
        .boxed()
}
