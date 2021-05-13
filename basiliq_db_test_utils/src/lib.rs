mod db;
mod init_values;

pub use db::{
    connect_to_management_pool, deinit_db, init_db, run_migrations, run_migrations_in_current_db,
    DATABASE_URL,
};
pub use init_values::{
    run as init_values, ARTICLES_IDS, COMMENTS_IDS, FAVORITE_COLOR_IDS, PEOPLES_IDS,
};
