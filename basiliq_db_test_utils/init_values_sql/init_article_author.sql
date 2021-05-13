INSERT INTO "people-article" ("people_id", "article_id")
    VALUES ($1, $2) RETURNING CONCAT_WS(',', "people_id", "article_id");
