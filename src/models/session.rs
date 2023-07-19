use crate::{models, schema::session, utils::now};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

const SESSION_DURATION_MINUTES: i64 = 60;

#[derive(Clone, Debug, Identifiable, Associations, Selectable, Queryable, AsChangeset)]
#[diesel(belongs_to(models::user::User))]
#[diesel(table_name = session)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub valid_until: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Session {
    pub fn for_update(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            valid_until: self.valid_until.clone(),
            created_at: self.created_at.clone(),
            updated_at: Some(now()),
            deleted_at: self.deleted_at.clone(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = session)]
pub struct NewSession {
    pub user_id: i32,
    pub valid_until: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewSession {
    pub fn new(user_id: i32) -> Self {
        NewSession {
            user_id: user_id,
            valid_until: now() + chrono::Duration::minutes(SESSION_DURATION_MINUTES),
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Session, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(
    conn: &mut PgConnection,
    new_session: &NewSession,
) -> Result<Session, diesel::result::Error> {
    diesel::insert_into(session::table)
        .values(new_session)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<Session>, diesel::result::Error> {
    session::table.load::<Session>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<Session, diesel::result::Error> {
    session::table
        .filter(session::id.eq(id))
        .first::<Session>(conn)
}

pub fn read_by_user_id(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Session, diesel::result::Error> {
    session::table
        .filter(session::user_id.eq(user_id))
        .filter(session::valid_until.gt(now()))
        .first::<Session>(conn)
}

pub fn delete(conn: &mut PgConnection, session: &Session) -> QueryResult<usize> {
    diesel::update(session)
        .set((session::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn delete_by_user_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
    diesel::update(session::dsl::session)
        .filter(session::user_id.eq(user_id))
        .filter(session::deleted_at.is_not_null())
        .set((session::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, session: &mut Session) -> QueryResult<usize> {
    diesel::update(session::table)
        .set(&session.for_update())
        .execute(conn)
}
