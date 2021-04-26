INSERT INTO peoples ("id", "first-name", "last-name", "age", "gender", "twitter", "favorite_color")
    VALUES ($1, 'Somebody', 'Wuhu', '34', 'F', '@randomhandle', $2) RETURNING id;
