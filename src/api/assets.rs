macro_rules! assets {
    () => {
        routes::assets::get_static()
    };
}

pub(crate) use assets;
