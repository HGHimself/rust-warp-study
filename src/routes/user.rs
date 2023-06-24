use crate::{models, server::Context};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("user").boxed()
}

pub fn get_by_id() -> BoxedFilter<(Context, models::user::User)> {
    warp::get()
        .and(path_prefix())
        .and(filters::ext::get::<Context>())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and_then(with_user)
        .untuple_one()
        .boxed()
}

async fn with_user(
    context: Context,
    id: i32,
) -> Result<(Context, models::user::User), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    log::info!("Looking for user with id of {}", id);
    let user = models::user::read_by_id(&mut conn, id).map_err(|_| reject::not_found())?;
    Ok((context, user))
}

pub fn create() -> BoxedFilter<(Context, models::user::User)> {
    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::NewUserApi>())
        .and_then(insert_new_user)
        .untuple_one()
        .boxed()
}

async fn insert_new_user(
    context: Context,
    new_user: models::user::NewUserApi,
) -> Result<(Context, models::user::User), warp::Rejection> {
    log::info!("Saving User");
    let mut conn = context.db_conn.get_conn();
    let user = models::user::NewUser::new(new_user)
        .insert(&mut conn)
        .map_err(|_| reject::reject())?;
    log::info!("Saved User");
    Ok((context, user))
}

pub fn create_form() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("create"))
        .and(warp::path::end())
        .boxed()
}
