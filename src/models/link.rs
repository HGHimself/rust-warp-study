use crate::{
    models,
    schema::{link, page_link},
    utils::now,
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

use super::page_link::PageLink;

#[derive(Debug, Identifiable, Selectable, Queryable, AsChangeset)]
#[diesel(table_name = link)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub creator_user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Link {
    pub fn for_update(&self) -> Self {
        Self {
            id: self.id,
            url: self.url.clone(),
            creator_user_id: self.creator_user_id.clone(),
            created_at: self.created_at.clone(),
            updated_at: Some(now()),
            deleted_at: self.deleted_at.clone(),
        }
    }

    pub fn inject_values(&self, string: &str) -> String {
        string
            .replace("{link.id}", &self.id.to_string())
            .replace("{link.url}", &self.url)
            .replace(
                "{link.favicon}",
                &(match url::Url::parse(&self.url) {
                    Err(_) => String::from(""),
                    Ok(mut url) => {
                        url.set_path("favicon.ico");
                        url.set_query(None);
                        url.into()
                    }
                }),
            )
            .replace("{link.creator_user_id}", &self.creator_user_id.to_string())
    }
}

#[derive(Deserialize)]
pub struct NewLinkApi {
    pub url: String,
    pub name: String,
    pub creator_user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = link)]
pub struct NewLink {
    pub url: String,
    pub creator_user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewLink {
    pub fn new(new_link: NewLinkApi) -> Self {
        NewLink {
            url: new_link.url,
            creator_user_id: new_link.creator_user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Link, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn create(conn: &mut PgConnection, new_link: &NewLink) -> Result<Link, diesel::result::Error> {
    diesel::insert_into(link::table)
        .values(new_link)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<Link>, diesel::result::Error> {
    link::table.load::<Link>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<Link, diesel::result::Error> {
    link::table.filter(link::id.eq(id)).first::<Link>(conn)
}

pub fn read_by_url(conn: &mut PgConnection, url: String) -> Result<Link, diesel::result::Error> {
    link::table.filter(link::url.eq(url)).first::<Link>(conn)
}

pub fn delete(conn: &mut PgConnection, link: &Link) -> QueryResult<usize> {
    diesel::update(link)
        .set((link::deleted_at.eq(Some(now())),))
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, link: &mut Link) -> QueryResult<usize> {
    diesel::update(link::table)
        .set(&link.for_update())
        .execute(conn)
}

pub fn read_links_by_page(
    conn: &mut PgConnection,
    page: &models::page::Page,
) -> Result<Vec<(Link, PageLink)>, diesel::result::Error> {
    models::page_link::PageLink::belonging_to(page)
        .inner_join(link::table)
        .select((Link::as_select(), PageLink::as_select()))
        .filter(page_link::deleted_at.is_null())
        .load::<(Link, PageLink)>(conn)
}
