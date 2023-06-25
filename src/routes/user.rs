use crate::{models, server::Context, NotFound};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("user").boxed()
}

pub fn signup() -> BoxedFilter<(Context, models::user::User)> {
    path_prefix()
        .and(warp::path("signup"))
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::NewUserApi>())
        .and_then(insert_new_user)
        .untuple_one()
        .boxed()
}

pub fn login() -> BoxedFilter<(Context, models::user::User)> {
    path_prefix()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::UserCredentialsApi>())
        .and_then(with_user_by_credentials)
        .untuple_one()
        .boxed()
}

pub fn get_by_id() -> BoxedFilter<(Context, models::user::User)> {
    path_prefix()
        .and(warp::get())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_user)
        .untuple_one()
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
            reject::reject()
        })?;
    log::info!("Saved User");
    Ok((context, user))
}

pub fn signup_form() -> BoxedFilter<()> {
    path_prefix()
        .and(warp::get())
        .and(warp::path("signup"))
        .and(warp::path::end())
        .boxed()
}

pub fn login_form() -> BoxedFilter<()> {
    path_prefix()
        .and(warp::get())
        .and(warp::path("login"))
        .and(warp::path::end())
        .boxed()
}

async fn with_user(
    context: Context,
    id: i32,
) -> Result<(Context, models::user::User), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for user with id of {}", id);
    let user = models::user::read_by_id(&mut conn, id).map_err(|_| reject::custom(NotFound))?;
    Ok((context, user))
}
