use crate::{
    schema::background,
    utils::{now, random},
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, Identifiable, Selectable, Queryable)]
#[diesel(table_name = background)]

pub struct Background {
    pub id: i32,
    pub count: i32,
    pub frequency: i32,
    pub x_amplitude: i32,
    pub y_amplitude: i32,
    pub x_multiplier: i32,
    pub y_multiplier: i32,
    pub color: i32,
    pub thickness: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Background {
    pub fn from_params(
        count: i32,
        frequency: i32,
        x_amplitude: i32,
        y_amplitude: i32,
        x_multiplier: i32,
        y_multiplier: i32,
        color: i32,
        thickness: i32,
    ) -> Self {
        Self {
            id: 0,
            count,
            frequency,
            x_amplitude,
            y_amplitude,
            x_multiplier,
            y_multiplier,
            color,
            thickness,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn to_call(&self) -> String {
        format!(
            "<script>showBackground({{
        count: {},
        frequency: {},
        xAmplitude: {},
        yAmplitude: {},
        xMultiplier: {},
        yMultiplier: {},
        color: {},
        thickness: {},
    }})</script>",
            self.count,
            self.frequency,
            self.x_amplitude,
            self.y_amplitude,
            self.x_multiplier,
            self.y_multiplier,
            self.color,
            self.thickness
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = background)]
pub struct NewBackground {
    pub count: i32,
    pub frequency: i32,
    pub x_amplitude: i32,
    pub y_amplitude: i32,
    pub x_multiplier: i32,
    pub y_multiplier: i32,
    pub color: i32,
    pub thickness: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl NewBackground {
    pub fn new(
        count: i32,
        frequency: i32,
        x_amplitude: i32,
        y_amplitude: i32,
        x_multiplier: i32,
        y_multiplier: i32,
        color: i32,
        thickness: i32,
    ) -> Self {
        Self {
            count,
            frequency,
            x_amplitude,
            y_amplitude,
            x_multiplier,
            y_multiplier,
            color,
            thickness,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> Result<Background, diesel::result::Error> {
        create(conn, self)
    }
}

pub fn random_bg() -> NewBackground {
    NewBackground::new(
        random(100, 50) as i32,
        random(15, 1) as i32,
        random(1500, 1000) as i32,
        random(1000, 600) as i32,
        random(15, 1) as i32,
        random(15, 1) as i32,
        random(2000, 3) as i32,
        random(75, 50) as i32,
    )
}

pub fn create(
    conn: &mut PgConnection,
    new_background: &NewBackground,
) -> Result<Background, diesel::result::Error> {
    diesel::insert_into(background::table)
        .values(new_background)
        .get_result(conn)
}

pub fn read(conn: &mut PgConnection) -> Result<Vec<Background>, diesel::result::Error> {
    background::table.load::<Background>(conn)
}

pub fn read_by_id(conn: &mut PgConnection, id: i32) -> Result<Background, diesel::result::Error> {
    background::table
        .filter(background::id.eq(id))
        .first::<Background>(conn)
}

pub fn background_random() -> String {
    Background::from_params(
        random(100, 50) as i32,
        random(15, 1) as i32,
        random(1500, 1000) as i32,
        random(1000, 600) as i32,
        random(15, 1) as i32,
        random(15, 1) as i32,
        random(2000, 3) as i32,
        random(75, 50) as i32,
    )
    .to_call()
}

// pub fn index() -> String {
//     Background::new(111, 8, 1690, 701, 9, 13, 1959, 50).to_call()
// }

pub fn index() -> String {
    Background::from_params(128, 7, 1703, 813, 11, 15, 704, 62).to_call()
}

pub fn login() -> String {
    Background::from_params(91, 3, 1346, 903, 7, 14, 1985, 53).to_call()
}

pub fn signup() -> String {
    Background::from_params(79, 7, 2066, 1165, 2, 13, 415, 101).to_call()
}
