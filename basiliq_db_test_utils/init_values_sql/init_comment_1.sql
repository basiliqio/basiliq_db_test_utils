INSERT INTO comments ("id", "author", "article", "body")
    VALUES ('59f58abd-c9db-4074-9c34-ac33e9c838ce', $1, $2, 'Wasnt convinced...') RETURNING id;
