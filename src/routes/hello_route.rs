use crate::{with_config, Config, NotEven, TesterJson};
use std::sync::Arc;
use warp::{filters::BoxedFilter, reject, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn sleepy() -> BoxedFilter<(u64,)> {
    warp::get()
        .and(warp::path("sleepy"))
        .and(warp::path::param::<u64>())
        .boxed()
}

pub fn hello() -> BoxedFilter<(u64, Arc<Config>)> {
    let config = Config { num: 14 };
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<u64>())
        .and_then(is_odd)
        .and_then(is_odd)
        .and(with_config(Arc::new(config)))
        .and_then(is_even)
        .untuple_one()
        .boxed()
}

pub fn hello_post() -> BoxedFilter<(TesterJson,)> {
    warp::post()
        .and(path_prefix())
        .and(warp::body::json())
        .boxed()
}

async fn is_odd(n: u64) -> Result<u64, warp::Rejection> {
    if n % 2 == 0 {
        Ok(n * 2)
    } else {
        Err(reject::custom(NotEven))
    }
}

async fn is_even(n: u64, config: Arc<Config>) -> Result<(u64, Arc<Config>), warp::Rejection> {
    if n % 2 == 0 {
        Ok(((n * 2), config))
    } else {
        Err(reject::custom(NotEven))
    }
}
