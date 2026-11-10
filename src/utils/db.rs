use crate::constants::env_key;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::sql_types::BigInt;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// Key for the Postgres advisory lock that serializes migrations. The value is arbitrary.
const MIGRATION_LOCK_KEY: i64 = 7_265_310_011;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}

/// Applies the pending embedded migrations. Panics if a migration fails, so the server never
/// starts on an incomplete schema.
pub fn run_migrations(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get a database connection");
    // Several instances can start at the same time (for example Vercel after a deploy).
    // The session lock makes the others wait until the first one has applied the migrations.
    lock(&mut conn, "pg_advisory_lock");
    let result = conn.run_pending_migrations(MIGRATIONS).map(|applied| applied.len());
    lock(&mut conn, "pg_advisory_unlock");
    match result {
        Ok(applied) => println!("applied {applied} pending migrations"),
        Err(err) => panic!("Failed to run migrations: {err}"),
    }
}

fn lock(conn: &mut PgConnection, function: &str) {
    diesel::sql_query(format!("SELECT {function}($1)"))
        .bind::<BigInt, _>(MIGRATION_LOCK_KEY)
        .execute(conn)
        .unwrap_or_else(|err| panic!("Failed to call {function}: {err}"));
}
