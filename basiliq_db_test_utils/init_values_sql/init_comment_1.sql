INSERT INTO comments ("id", "author", "article", "body")
    VALUES ($1, $2, $3, 'Wasnt convinced...') RETURNING id;
