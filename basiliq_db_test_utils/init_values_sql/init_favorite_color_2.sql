INSERT INTO favorite_color ("id", "color")
    VALUES ($1, 'blue') RETURNING id;

