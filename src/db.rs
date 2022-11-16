use crate::errors::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use log::info;

type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PostgresPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

lazy_static! {
    static ref DB_POOL: PostgresPool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create pool.")
    };
}

pub fn init() {
    info!("Initializing database");
    lazy_static::initialize(&DB_POOL);
    let mut conn = connection().expect("Failed to get connection from pool");
    MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS)
        .expect("Failed to run migrations");
}

pub fn connection() -> Result<PostgresPooledConnection, ApiError> {
    DB_POOL
        .get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
