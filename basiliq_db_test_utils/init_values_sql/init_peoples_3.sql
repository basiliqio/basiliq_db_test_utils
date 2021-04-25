INSERT INTO peoples ("id", "first-name", "last-name", "favorite_color")
    VALUES ('961e543a-4b22-4d48-a8e5-c1eafada950f', 'AAAAAAAA', 'BBBBBBBBB', $1) RETURNING id;
