use crate::{
    models,
    schema::{background, page, page_link},
    utils::{now, sanitize_html},
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, Identifiable, Selectable, Queryable, AsChangeset)]
#[diesel(belongs_to(models::background::Background))]
#[diesel(table_name = page)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub background_id: i32,
}

#[derive(Clone, Debug)]
pub struct ExpandedPage {
    pub page: Page,
    pub background: models::background::Background,
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
            background_id: self.background_id.clone(),
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
    pub background_id: i32,
}

impl NewPage {
    pub fn new(new_page: NewPageApi, user_id: i32, background_id: i32) -> Self {
        NewPage {
            name: sanitize_html(&new_page.name),
            description: sanitize_html(&new_page.description),
            user_id: user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            background_id: background_id,
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

pub fn expand(page: Page, background: models::background::Background) -> ExpandedPage {
    ExpandedPage {
        page: page,
        background: background,
    }
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<ExpandedPage, diesel::result::Error> {
    page::table
        .inner_join(background::table.on(page::background_id.eq(background::id)))
        .filter(page::id.eq(id))
        .filter(page::deleted_at.is_null())
        .select((
            Page::as_select(),
            models::background::Background::as_select(),
        ))
        .first(conn)
        .map(|(page, background)| expand(page, background))
}

pub fn read_by_id_and_user_id(
    conn: &mut PgConnection,
    id: i32,
    user_id: i32,
) -> Result<ExpandedPage, diesel::result::Error> {
    page::table
        .inner_join(background::table.on(page::background_id.eq(background::id)))
        .filter(page::id.eq(id))
        .filter(page::deleted_at.is_null())
        .filter(page::user_id.eq(user_id))
        .select((
            Page::as_select(),
            models::background::Background::as_select(),
        ))
        .first(conn)
        .map(|(page, background)| expand(page, background))
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
        .filter(page::deleted_at.is_null())
        .load::<Page>(conn)
}

pub fn read_pages_by_link(
    conn: &mut PgConnection,
    link: &models::link::Link,
) -> Result<Vec<Page>, diesel::result::Error> {
    models::page_link::PageLink::belonging_to(link)
        .inner_join(page::table)
        .select(Page::as_select())
        .filter(page_link::deleted_at.is_null())
        .filter(page::deleted_at.is_null())
        .load(conn)
}

pub fn get_count_of_pages_per_user(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    page::table
        .filter(page::user_id.eq(user_id))
        .filter(page::deleted_at.is_null())
        .count()
        .execute(conn)
}