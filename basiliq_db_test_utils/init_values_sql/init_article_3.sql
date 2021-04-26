INSERT INTO articles ("id", "title", "body")
    VALUES ($1, 'Oh my g**', 'Yeah I know ! Right ?!') RETURNING id;
