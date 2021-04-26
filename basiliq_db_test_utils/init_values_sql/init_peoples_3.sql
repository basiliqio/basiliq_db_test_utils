INSERT INTO peoples ("id", "first-name", "last-name", "favorite_color")
    VALUES ($1, 'AAAAAAAA', 'BBBBBBBBB', $2) RETURNING id;
