macro_rules! webauthn {
    ($webauthn:expr) => {
        routes::webauthn::register($webauthn).and_then(handlers::webauthn::register)
    };
}

pub(crate) use webauthn;
