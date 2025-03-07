use crate::{
    models,
    schema::{link, page_link},
    utils::{get_metadata_from_url, now, sanitize_html},
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
    pub img_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
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
            img_url: self.img_url.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
        }
    }

    pub fn inject_values(&self, string: &str) -> String {
        string
            .replace("{link.id}", &self.id.to_string())
            .replace("{link.url}", &self.url)
            .replace(
                "{link.img_url}",
                match &self.img_url {
                    Some(url) => &url,
                    None => "/missing.png",
                },
            )
            .replace(
                "{link.title}",
                match &self.title {
                    Some(title) => {
                        if "" == title {
                            &self.url
                        } else {
                            title
                        }
                    }
                    None => &self.url,
                },
            )
            .replace(
                "{link.description}",
                match &self.description {
                    Some(description) => &description,
                    None => "",
                },
            )
            .replace(
                "{link.favicon}",
                &(match url::Url::parse(&self.url) {
                    Err(_) => String::from("/missing.png"),
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
}

#[derive(Deserialize)]
pub struct AddLinkToPageApi {
    pub page_id: i32,
    pub name: String,
    pub url: String,
}

impl Into<NewLinkApi> for AddLinkToPageApi {
    fn into(self) -> NewLinkApi {
        NewLinkApi {
            name: self.name,
            url: self.url,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = link)]
pub struct NewLink {
    pub url: String,
    pub creator_user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub img_url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl NewLink {
    pub fn new(new_link: NewLinkApi, creator_user_id: i32) -> Self {
        let metadata = get_metadata_from_url(&new_link.url);
        NewLink {
            url: sanitize_html(&new_link.url),
            creator_user_id: creator_user_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            title: metadata.title,
            description: metadata.description,
            img_url: metadata.img_url,
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

pub fn get_count_of_links(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    link::table
        .count()
        .filter(link::deleted_at.is_null())
        .get_result(conn)
        .map(|v: i64| v as usize)
}
