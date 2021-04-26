INSERT INTO articles ("id", "title", "body")
    VALUES ($1, 'How to dead', 'Yes') RETURNING id;
