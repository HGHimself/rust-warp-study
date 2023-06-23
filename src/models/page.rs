use crate::{
    models,
    routes::page::get_by_id,
    schema::{link, page, page_link},
    utils::now,
};
use chrono::naive::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Identifiable, Selectable, Queryable, AsChangeset)]
#[diesel(table_name = page)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Page {
    pub fn for_update(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            user_id: self.user_id,
            description: self.description.clone(),
            created_at: self.created_at.clone(),
            updated_at: Some(now()),
            deleted_at: self.deleted_at.clone(),
        }
    }

    pub fn inject_values(&self, string: &str) -> String {
        string
            .replace("{page.id}", &self.id.to_string())
            .replace("{page.name}", &self.name)
            .replace("{page.description}", &self.description)
    }
}

#[derive(Deserialize)]
pub struct NewPageApi {
    pub name: String,
    pub description: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = page)]
pub struct NewPage {
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewPage {
    pub fn new(new_page: NewPageApi) -> Self {
        NewPage {
            name: new_page.name,
            description: new_page.description,
            user_id: new_page.user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Page, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(conn: &mut PgConnection, new_page: &NewPage) -> Result<Page, diesel::result::Error> {
    diesel::insert_into(page::table)
        .values(new_page)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<Page>, diesel::result::Error> {
    page::table.load::<Page>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<Page, diesel::result::Error> {
    page::table.filter(page::id.eq(id)).first::<Page>(conn)
}

pub fn delete(conn: &mut PgConnection, page: &Page) -> QueryResult<usize> {
    diesel::update(page)
        .set((page::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, page: &mut Page) -> QueryResult<usize> {
    diesel::update(page::table)
        .set(&page.for_update())
        .execute(conn)
}

pub fn read_pages_by_user_id(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<Page>, diesel::result::Error> {
    page::table
        .filter(page::user_id.eq(user_id))
        .load::<Page>(conn)
}

pub fn read_pages_by_link(
    conn: &mut PgConnection,
    link: &models::link::Link,
) -> Result<Vec<Page>, diesel::result::Error> {
    models::page_link::PageLink::belonging_to(link)
        .inner_join(page::table)
        .select(Page::as_select())
        .load(conn)
}
