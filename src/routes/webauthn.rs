use crate::{services::webauthn::Webauthn, with_webauthn, NotEven, TesterJson};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("auth").boxed()
}

pub fn register(webauthn: Arc<Webauthn>) -> BoxedFilter<(Arc<Webauthn>, String)> {
    warp::post()
        .and(path_prefix())
        .and(with_webauthn(webauthn))
        .and(warp::path::param())
        .boxed()
}
