mod db;
mod init_values;

pub use db::{connect_to_management_pool, deinit_db, init_db, DATABASE_URL};
pub use init_values::{
    ARTICLES_IDS, COMMENTS_IDS, FAVORITE_COLOR_IDS, PEOPLES_IDS, PEOPLE_ARTICLE_ID,
};
