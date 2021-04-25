INSERT INTO peoples ("id", "first-name", "last-name", "age", "gender", "twitter", "favorite_color")
    VALUES ('777cc565-c66b-4942-ab44-8fc5f194b804', 'Somebody', 'Wuhu', '34', 'F', '@randomhandle', $1) RETURNING id;
