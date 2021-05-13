use lazy_static::lazy_static;
use std::collections::BTreeMap;
use uuid::Uuid;

lazy_static! {
    pub static ref FAVORITE_COLOR_IDS: [Uuid; 3] = [
        Uuid::parse_str("9302b136-69ef-4f0d-b907-10593ff7e578").unwrap(),
        Uuid::parse_str("aad8943c-ef9e-4dd2-8b6b-b2b1e61d17eb").unwrap(),
        Uuid::parse_str("90f99944-3340-4f87-b07d-b2ddf64be1ff").unwrap()
    ];
    pub static ref PEOPLES_IDS: [Uuid; 3] = [
        Uuid::parse_str("1649b1e9-8a5f-4f52-b331-c07ce3bccc6f").unwrap(),
        Uuid::parse_str("777cc565-c66b-4942-ab44-8fc5f194b804").unwrap(),
        Uuid::parse_str("961e543a-4b22-4d48-a8e5-c1eafada950f").unwrap()
    ];
    pub static ref ARTICLES_IDS: [Uuid; 3] = [
        Uuid::parse_str("fdf715dd-8772-498c-8196-6f4ccb64edef").unwrap(),
        Uuid::parse_str("46c4fe50-8c56-4f26-935e-56ccfa496bb5").unwrap(),
        Uuid::parse_str("2dbf5d1a-b029-4456-af6b-339c75b1089c").unwrap()
    ];
    pub static ref COMMENTS_IDS: [Uuid; 3] = [
        Uuid::parse_str("59f58abd-c9db-4074-9c34-ac33e9c838ce").unwrap(),
        Uuid::parse_str("6ae9938f-d490-4707-b138-770c2a52465f").unwrap(),
        Uuid::parse_str("c2add83b-6f58-45a2-bf62-3ebc05c46192").unwrap()
    ];
}

const FAVORITE_COLOR_INIT_FILES: [&str; 3] = [
    include_str!("../init_values_sql/init_favorite_color_1.sql"),
    include_str!("../init_values_sql/init_favorite_color_2.sql"),
    include_str!("../init_values_sql/init_favorite_color_3.sql"),
];

const PEOPLES_INIT_FILES: [&str; 3] = [
    include_str!("../init_values_sql/init_peoples_1.sql"),
    include_str!("../init_values_sql/init_peoples_2.sql"),
    include_str!("../init_values_sql/init_peoples_3.sql"),
];

const ARTICLE_INIT_FILES: [&str; 3] = [
    include_str!("../init_values_sql/init_article_1.sql"),
    include_str!("../init_values_sql/init_article_2.sql"),
    include_str!("../init_values_sql/init_article_3.sql"),
];

const COMMENT_INIT_FILES: [&str; 3] = [
    include_str!("../init_values_sql/init_comment_1.sql"),
    include_str!("../init_values_sql/init_comment_2.sql"),
    include_str!("../init_values_sql/init_comment_3.sql"),
];

const PEOPLE_ARTICLE: &str = include_str!("../init_values_sql/init_article_author.sql");

async fn init_favorite_colors(pool: &mut sqlx::PgPool) -> Vec<Uuid> {
    let mut favorite_color_id: Vec<Uuid> = Vec::with_capacity(3);
    for (idx, files) in FAVORITE_COLOR_INIT_FILES.iter().enumerate() {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(FAVORITE_COLOR_IDS[idx])
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        favorite_color_id.push(id.0);
    }
    favorite_color_id
}

async fn init_peoples(pool: &mut sqlx::PgPool, favorite_colors: &[Uuid]) -> Vec<Uuid> {
    let mut peoples_id: Vec<Uuid> = Vec::with_capacity(3);
    for (idx, (files, favorite_color_id)) in
        PEOPLES_INIT_FILES.iter().zip(favorite_colors).enumerate()
    {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(PEOPLES_IDS[idx])
            .bind(favorite_color_id)
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        peoples_id.push(id.0);
    }
    peoples_id
}

async fn init_articles(pool: &mut sqlx::PgPool) -> Vec<Uuid> {
    let mut article_id: Vec<Uuid> = Vec::with_capacity(3);
    for (idx, files) in ARTICLE_INIT_FILES.iter().enumerate() {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(ARTICLES_IDS[idx])
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        article_id.push(id.0);
    }
    article_id
}

async fn init_comments(
    pool: &mut sqlx::PgPool,
    peoples_id: &[Uuid],
    articles_id: &[Uuid],
) -> Vec<Uuid> {
    let mut comment_id: Vec<Uuid> = Vec::with_capacity(3);
    for (idx, (files, article_id)) in COMMENT_INIT_FILES.iter().zip(articles_id).enumerate() {
        let id: (Uuid,) = sqlx::query_as(files)
            .bind(COMMENTS_IDS[idx])
            .bind(peoples_id[idx % 2])
            .bind(article_id)
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        comment_id.push(id.0);
    }
    comment_id
}

async fn init_link_article_author(
    pool: &mut sqlx::PgPool,
    peoples_id: &[Uuid],
    articles_id: &[Uuid],
) -> Vec<String> {
    let mut article_author_id: Vec<String> = Vec::with_capacity(3);
    for (idx, article_id) in articles_id.iter().enumerate() {
        let id: (String,) = sqlx::query_as(PEOPLE_ARTICLE)
            .bind(peoples_id[idx % 2])
            .bind(article_id)
            .fetch_one(&mut pool.acquire().await.unwrap())
            .await
            .unwrap();
        article_author_id.push(id.0);
    }
    let id: (String,) = sqlx::query_as(PEOPLE_ARTICLE)
        .bind(peoples_id[1])
        .bind(articles_id[0])
        .fetch_one(&mut pool.acquire().await.unwrap())
        .await
        .unwrap();
    article_author_id.push(id.0);
    article_author_id
}

pub async fn run(pool: &mut sqlx::PgPool) -> BTreeMap<String, Vec<String>> {
    let mut res: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let favorite_color_id = init_favorite_colors(&mut *pool).await;
    let article_id = init_articles(&mut *pool).await;
    let peoples_id = init_peoples(&mut *pool, &&favorite_color_id).await;
    let comment_id = init_comments(&mut *pool, &peoples_id, &article_id).await;
    let people_article = init_link_article_author(&mut *pool, &peoples_id, &article_id).await;

    res.insert("favorite_color".to_string(), favorite_color_id.into_iter().map(|x| x.to_string()).collect());
    res.insert("peoples".to_string(), peoples_id.into_iter().map(|x| x.to_string()).collect());
    res.insert("articles".to_string(), article_id.into_iter().map(|x| x.to_string()).collect());
    res.insert("comments".to_string(), comment_id.into_iter().map(|x| x.to_string()).collect());
    res.insert("people-article".to_string(), people_article);
    res
}
