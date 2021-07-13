use crate::NotEven;
use warp::{filters::BoxedFilter, reject, Filter, Rejection};

// 1. "hello"
fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn hello() -> BoxedFilter<(u64,)> {
    log::info!("Ayo we got a hit at hello");
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .and(warp::path::param::<u64>()) // 5.
        .and_then(is_even)
        .boxed()
}

async fn is_even(n: u64) -> Result<u64, warp::Rejection> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(reject::custom(NotEven))
    }
}
