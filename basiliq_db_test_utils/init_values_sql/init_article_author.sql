INSERT INTO "people-article" ("id", "people_id", "article_id")
    VALUES ($1, $2, $3) RETURNING id;
