INSERT INTO peoples ("id", "first-name", "last-name", "age", "gender", "favorite_color")
    VALUES ('1649b1e9-8a5f-4f52-b331-c07ce3bccc6f', 'Francis', 'Le Roy', '22', 'M', $1) RETURNING id;

