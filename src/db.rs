use diesel;
use r2d2;
use r2d2_diesel;
use std;

/// Type alias for the r2d2 connection pool. Use this as a State<T> parameter
/// in handlers that need a database connection.
pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>;

/// Creates the database connection pool
pub fn establish_connection_pool() -> ConnectionPool {
  let config = r2d2::Config::default();
  let connection_manager = r2d2_diesel::ConnectionManager::<diesel::pg::PgConnection>::new(std::env::var("DATABASE_URL").unwrap());
  r2d2::Pool::new(config, connection_manager).unwrap()
}
