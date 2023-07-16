use crate::{
    models,
    schema::page_link,
    utils::{now, sanitize_html},
};
use chrono::naive::NaiveDateTime;
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
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl PageLink {
    pub fn inject_values(&self, string: &str) -> String {
        string.replace("{page_link.name}", &self.name.to_string())
    }
}

#[derive(Insertable)]
#[diesel(table_name = page_link)]
pub struct NewPageLink {
    pub page_id: i32,
    pub link_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewPageLink {
    pub fn new(page_id: i32, link_id: i32, name: String) -> Self {
        Self {
            page_id: page_id,
            link_id: link_id,
            name: sanitize_html(&name),
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

pub fn remove_link_by_page_id_and_link_id(
    conn: &mut PgConnection,
    page_id: i32,
    link_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(page_link::table)
        .set((page_link::deleted_at.eq(Some(now())),))
        .filter(page_link::page_id.eq(page_id))
        .filter(page_link::link_id.eq(link_id))
        .execute(conn)
}

pub fn get_count_of_links_per_page(
    conn: &mut PgConnection,
    page_id: i32,
) -> Result<usize, diesel::result::Error> {
    page_link::table
        .filter(page_link::page_id.eq(page_id))
        .filter(page_link::deleted_at.is_null())
        .count()
        .execute(conn)
}
