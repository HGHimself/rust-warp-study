use crate::{schema::user, utils::now};
use chrono::naive::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn for_update(&self) -> Self {
        Self {
            id: self.id,
            first_name: self.first_name.clone(),
            middle_name: self.middle_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            birthday: self.birthday.clone(),
            created_at: self.created_at.clone(),
            updated_at: Some(now()),
            deleted_at: self.deleted_at.clone(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn inject_values(&self, string: &str) -> String {
        string
            .replace("{user.id}", &self.id.to_string())
            .replace("{user.first_name}", &self.first_name)
            .replace(
                "{user.middle_name}",
                &self.middle_name.clone().unwrap_or(String::from("")),
            )
            .replace("{user.last_name}", &self.last_name)
            .replace("{user.email}", &self.email)
            .replace("{user.birthday}", &self.birthday.to_string())
    }
}

#[derive(Deserialize)]
pub struct NewUserApi {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub birthday: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewUser {
    pub fn new(new_user: NewUserApi) -> Self {
        NewUser {
            first_name: new_user.first_name,
            middle_name: new_user.middle_name,
            last_name: new_user.last_name,
            email: new_user.email,
            birthday: new_user.birthday,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
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
    user::table.filter(user::id.eq(id)).first::<User>(conn)
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
