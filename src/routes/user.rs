use crate::{models, server::Context, DuplicateResource, NotAuthorized, NotFound};
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

pub fn signup() -> BoxedFilter<(Context, models::user::User, models::session::Session)> {
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

pub fn login() -> BoxedFilter<(Context, models::user::User, models::session::Session)> {
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

pub fn get_by_cookie() -> BoxedFilter<(Context, models::user::User, models::session::Session)> {
    warp::path::end()
        .and(warp::get())
        .and(authenticate_cookie())
        .boxed()
}

async fn with_user_by_credentials(
    context: Context,
    credentials: models::user::UserCredentialsApi,
) -> Result<(Context, models::user::User), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for user {}", credentials.username);
    let user = models::user::read_by_credentials(&mut conn, credentials)
        .map_err(|_| reject::custom(NotFound))?;
    Ok((context, user))
}

async fn insert_new_user(
    context: Context,
    new_user: models::user::NewUserApi,
) -> Result<(Context, models::user::User), warp::Rejection> {
    log::info!("Saving User");
    let mut conn = context.db_conn.get_conn();
    let user = models::user::NewUser::new(new_user.into())
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
    Ok((context, user))
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
) -> Result<(Context, models::user::User, models::session::Session), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    let session = models::session::NewSession::new(user.id)
        .insert(&mut conn)
        .map_err(|_| warp::reject::custom(DuplicateResource))?;

    Ok((context, user, session))
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
) -> Result<(Context, models::user::User, models::session::Session), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let (user, session) = models::user::read_user_by_session(&mut conn, session_id)
        .map_err(|_| warp::reject::custom(NotAuthorized))?;

    Ok((context, user, session))
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

pub fn authenticate_cookie() -> BoxedFilter<(Context, models::user::User, models::session::Session)>
{
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
