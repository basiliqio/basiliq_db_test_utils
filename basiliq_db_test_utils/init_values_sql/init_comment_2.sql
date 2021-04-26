INSERT INTO comments ("id", "author", "article", "body")
    VALUES ($1, $2, $3, 'It was great !') RETURNING id;
