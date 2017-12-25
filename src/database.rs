use std::time::Duration;

use diesel;
use r2d2::{self, PooledConnection};
use r2d2_diesel;

pub use diesel::pg::PgConnection;
pub use diesel::Connection;

pub type PgPool = r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>;

pub fn make_pool(database_url: &str) -> PgPool {
    let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("create database pool")
}
