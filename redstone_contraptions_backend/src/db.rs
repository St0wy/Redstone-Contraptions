use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_db_pool() -> Pool {
    // Get the url of the db with user and password
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // Create a new db connection manager
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    // Create a new connection pool (so that a new connection is not create for each user)
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
