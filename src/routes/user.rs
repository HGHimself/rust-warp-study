use crate::{
    models::{self, user::ExpandedUser},
    server::Context,
    utils::now,
    DuplicateResource, NotAuthorized, NotFound, OldCookie,
};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

pub fn logout() -> BoxedFilter<()> {
    warp::path("logout")
        .and(warp::path::end())
        .and(warp::get())
        .and(read_cookie())
        .and_then(clear_session)
        .untuple_one()
        .boxed()
}

pub fn signup() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::NewUserApi>())
        .and_then(insert_new_user)
        .untuple_one()
        .and_then(with_new_session)
        .untuple_one()
        .boxed()
}

pub fn login() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::UserCredentialsApi>())
        .and_then(with_user_by_credentials)
        .untuple_one()
        .and_then(with_new_session)
        .untuple_one()
        .boxed()
}

pub fn get_by_cookie() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::path::end()
        .and(warp::get())
        .and(authenticate_cookie())
        .boxed()
}

async fn with_user_by_credentials(
    context: Context,
    credentials: models::user::UserCredentialsApi,
) -> Result<(Context, models::user::User, models::background::Background), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for user {}", credentials.username);
    let (user, background) = models::user::read_by_credentials(&mut conn, credentials)
        .map_err(|_| reject::custom(NotFound))?;
    Ok((context, user, background))
}

async fn insert_new_user(
    context: Context,
    new_user: models::user::NewUserApi,
) -> Result<(Context, models::user::User, models::background::Background), warp::Rejection> {
    log::info!("Saving User");
    let mut conn = context.db_conn.get_conn();
    let background = models::background::random_bg()
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            warp::reject()
        })?;

    let user = models::user::NewUser::new(new_user.into(), background.id)
        .insert(&mut conn)
        .map_err(|e| {
            log::error!("{:?}", e);
            match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    reject::custom(DuplicateResource)
                }
                _ => reject::reject(),
            }
        })?;
    log::info!("Saved User");
    Ok((context, user, background))
}

pub fn signup_form() -> BoxedFilter<()> {
    warp::path("signup")
        .and(warp::path::end())
        .and(warp::get())
        .boxed()
}

pub fn login_form() -> BoxedFilter<()> {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::get())
        .boxed()
}

async fn with_new_session(
    context: Context,
    user: models::user::User,
    background: models::background::Background,
) -> Result<(Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    let session = models::session::NewSession::new(user.id)
        .insert(&mut conn)
        .map_err(|_| warp::reject::custom(DuplicateResource))?;

    let expanded_user = ExpandedUser {
        user,
        background,
        session,
    };
    Ok((context, expanded_user))
}

async fn clear_session(
    context: Context,
    session: models::session::Session,
) -> Result<(), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    models::session::delete(&mut conn, &session).map_err(|_| warp::reject::custom(NotFound))?;
    Ok(())
}

async fn with_user_from_cookie(
    context: Context,
    session_id: i32,
) -> Result<(Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Session ID: {}", session_id);
    let expanded_user = models::user::read_user_by_session(&mut conn, session_id)
        .map_err(|_| warp::reject::custom(NotAuthorized))?;
    log::info!(
        "Recognized user {:?} from {:?}",
        expanded_user.user,
        expanded_user.session
    );

    if expanded_user.session.valid_until < now() {
        models::session::delete(&mut conn, &expanded_user.session)
            .map_err(|_| warp::reject::custom(NotFound))?;
        return Err(warp::reject::custom(OldCookie));
    }

    Ok((context, expanded_user))
}

async fn with_session_from_cookie(
    context: Context,
    session_id: i32,
) -> Result<(Context, models::session::Session), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let session = models::session::read_by_id(&mut conn, session_id)
        .map_err(|_| warp::reject::custom(NotFound))?;

    Ok((context, session))
}

pub fn authenticate_cookie() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and(warp::cookie("session"))
        .and_then(with_user_from_cookie)
        .untuple_one()
        .boxed()
}

pub fn read_cookie() -> BoxedFilter<(Context, models::session::Session)> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and(warp::cookie("session"))
        .and_then(with_session_from_cookie)
        .untuple_one()
        .boxed()
}
