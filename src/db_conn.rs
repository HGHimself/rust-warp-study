use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use log::info;

#[derive(Clone, Debug)]
pub struct DbConn {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbConn {
    pub fn new(conn_string: &str) -> Self {
        info!("💾 Connecting to Database!");
        let manager = ConnectionManager::<PgConnection>::new(conn_string);
        let pool = Pool::new(manager).unwrap();

        DbConn { pool }
    }

    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}

pub fn establish_test_connection() -> PgConnection {
    let database_url = crate::config::db_test_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
