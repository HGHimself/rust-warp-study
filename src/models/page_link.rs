use crate::{models, schema::page_link, utils::now};
use chrono::naive::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(models::page::Page))]
#[diesel(belongs_to(models::link::Link))]
#[diesel(table_name = page_link)]
#[diesel(primary_key(page_id, link_id))]
pub struct PageLink {
    pub id: i32,
    pub page_id: i32,
    pub link_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = page_link)]
pub struct NewPageLink {
    pub page_id: i32,
    pub link_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewPageLink {
    pub fn new(page_id: i32, link_id: i32) -> Self {
        Self {
            page_id: page_id,
            link_id: link_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<PageLink, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(
    conn: &mut PgConnection,
    new_page_link: &NewPageLink,
) -> Result<PageLink, diesel::result::Error> {
    diesel::insert_into(page_link::table)
        .values(new_page_link)
        .get_result(conn)
}
