macro_rules! assets {
    () => {
        routes::assets::get_static()
            .recover(handle_rejection)
            .with(warp::trace::named("assets"))
    };
}

pub(crate) use assets;
