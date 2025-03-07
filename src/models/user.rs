use crate::{
    models,
    schema::{background, session, user},
    utils::{encrypt, now, sanitize_html, verify},
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(belongs_to(models::background::Background))]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub background_id: i32,
}

impl User {
    pub fn for_update(&self) -> Self {
        Self {
            id: self.id,
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: self.created_at.clone(),
            updated_at: Some(now()),
            deleted_at: self.deleted_at.clone(),
            background_id: self.background_id,
        }
    }

    pub fn inject_values(&self, string: &str) -> String {
        string
            .replace("{user.id}", &self.id.to_string())
            .replace("{user.username}", &self.username)
    }
}

#[derive(Deserialize)]
pub struct NewUserApi {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

impl Into<UserCredentialsEncrypted> for NewUserApi {
    fn into(self) -> UserCredentialsEncrypted {
        UserCredentialsEncrypted {
            username: sanitize_html(&self.username),
            password: encrypt(&self.password),
        }
    }
}

#[derive(Deserialize)]
pub struct UserCredentialsApi {
    pub username: String,
    pub password: String,
}

impl Into<UserCredentialsEncrypted> for UserCredentialsApi {
    fn into(self) -> UserCredentialsEncrypted {
        UserCredentialsEncrypted {
            username: self.username,
            password: encrypt(&self.password),
        }
    }
}

pub struct UserCredentialsEncrypted {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct ExpandedUser {
    pub user: User,
    pub background: models::background::Background,
    pub session: models::session::Session,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub background_id: i32,
}

impl NewUser {
    pub fn new(new_user: UserCredentialsEncrypted, background_id: i32) -> Self {
        NewUser {
            username: new_user.username,
            password: new_user.password,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            background_id: background_id,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<User, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(user::table)
        .values(new_user)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    user::table.load::<User>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    user::table
        .filter(user::id.eq(id))
        .filter(user::deleted_at.is_null())
        .first::<User>(conn)
}

pub fn read_by_credentials(
    conn: &mut PgConnection,
    credentials: UserCredentialsApi,
) -> Result<(User, models::background::Background), diesel::result::Error> {
    let (user, background) = user::table
        .inner_join(background::table.on(user::background_id.eq(background::id)))
        .filter(user::username.eq(credentials.username))
        .filter(user::deleted_at.is_null())
        .select((
            User::as_select(),
            models::background::Background::as_select(),
        ))
        .first(conn)?;

    if verify(&credentials.password, &user.password) {
        Ok((user, background))
    } else {
        Err(diesel::NotFound)
    }
}

pub fn delete(conn: &mut PgConnection, user: &User) -> QueryResult<usize> {
    diesel::update(user)
        .set((user::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, user: &mut User) -> QueryResult<usize> {
    diesel::update(user::table)
        .set(&user.for_update())
        .execute(conn)
}

pub fn read_user_by_session(
    conn: &mut PgConnection,
    session_id: i32,
) -> Result<ExpandedUser, diesel::result::Error> {
    let r = user::table
        .inner_join(background::table.on(user::background_id.eq(background::id)))
        .inner_join(session::table.on(user::id.eq(session::user_id)))
        // .filter(session::valid_until.gt(now()))
        .filter(session::deleted_at.is_null())
        .filter(session::id.eq(session_id))
        .filter(user::deleted_at.is_null())
        .select((
            User::as_select(),
            models::session::Session::as_select(),
            models::background::Background::as_select(),
        ))
        .first(conn);

    r.map(|(user, session, background)| ExpandedUser {
        user,
        session,
        background,
    })
}

pub fn cleanup_table(conn: &mut PgConnection) {
    diesel::delete(user::table).execute(conn).unwrap();
}
