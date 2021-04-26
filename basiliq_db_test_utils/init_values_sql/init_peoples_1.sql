INSERT INTO peoples ("id", "first-name", "last-name", "age", "gender", "favorite_color")
    VALUES ($1, 'Francis', 'Le Roy', '22', 'M', $2) RETURNING id;

