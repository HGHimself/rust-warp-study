macro_rules! assets {
    () => {
        routes::assets::get_static().with(warp::trace::named("assets"))
    };
}

pub(crate) use assets;
