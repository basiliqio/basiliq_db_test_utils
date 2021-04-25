use lazy_static::lazy_static;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

mod embedded_migrations {
    use refinery::embed_migrations;
    embed_migrations!("./tests_migrations");
}

lazy_static! {
    pub static ref BASILIQ_DATABASE_URL: String =
        std::env::var("BASILIQ_DATABASE_URL").expect("the database url to be set");
    pub static ref BASILIQ_DEFAULT_DATABASE: String = format!("basiliq_test_{}", Uuid::new_v4());
    pub static ref BASILIQ_DEFAULT_DATABASE_INIT: Mutex<bool> = Mutex::new(false);
}

pub async fn run_migrations(db_name: &str) {
    let mut config = refinery::config::Config::from_env_var("BASILIQ_DATABASE_URL")
        .expect("to parse the basiliq database url")
        .set_db_name(db_name);
    embedded_migrations::migrations::runner()
        .run_async(&mut config)
        .await
        .expect("to apply migrations");
}
pub fn connect_to_management_pool() -> sqlx::PgPool {
    let num = num_cpus::get();

    sqlx::pool::PoolOptions::new()
        .min_connections(1)
        .max_connections(num as u32)
        .connect_lazy(&BASILIQ_DATABASE_URL)
        .expect("to initialize the management Postgres connection pool")
}
pub async fn init_db(
    should_run_migrations: bool,
    should_init_values: bool,
) -> (String, sqlx::PgPool) {
    let management_pool = connect_to_management_pool();
    {
        let mut init_bool = BASILIQ_DEFAULT_DATABASE_INIT
            .lock()
            .expect("the database management mutex is poisoned");
        {
            if !*init_bool {
                sqlx::query(
                    format!("CREATE DATABASE \"{}\";", BASILIQ_DEFAULT_DATABASE.as_str()).as_str(),
                )
                .execute(&management_pool)
                .await
                .expect("to create a new database");
                run_migrations(BASILIQ_DEFAULT_DATABASE.as_str()).await;
                *init_bool = true;
            }
        }
    }
    let db_name = format!("basiliq_test_{}", Uuid::new_v4());
    if should_run_migrations {
        sqlx::query(
            format!(
                "CREATE DATABASE \"{}\" WITH TEMPLATE \"{}\";",
                db_name.as_str(),
                BASILIQ_DEFAULT_DATABASE.as_str()
            )
            .as_str(),
        )
        .execute(&management_pool)
        .await
        .expect("to create a new database");
    } else {
        sqlx::query(format!("CREATE DATABASE \"{}\";", db_name.as_str(),).as_str())
            .execute(&management_pool)
            .await
            .expect("to create a new database");
    }
    let conn_opt = sqlx::postgres::PgConnectOptions::from_str(&BASILIQ_DATABASE_URL)
        .expect("to parse the basiliq database url")
        .database(db_name.as_str());
    let mut pool = sqlx::pool::PoolOptions::new()
        .test_before_acquire(false)
        .min_connections(1)
        .max_connections(3)
        .connect_lazy_with(conn_opt);
    if !should_run_migrations {
        sqlx::query("CREATE EXTENSION \"uuid-ossp\";")
            .execute(&pool)
            .await
            .expect("to create uuid extension in the database");
    } else if should_init_values {
        super::init_values::run(&mut pool).await;
    }
    (db_name, pool)
}

pub async fn deinit_db(db_id: String) {
    let management_pool = connect_to_management_pool();
    sqlx::query(format!("DROP DATABASE \"{}\"", db_id.as_str(),).as_str())
        .execute(&management_pool)
        .await
        .expect("to delete a new database");
}
