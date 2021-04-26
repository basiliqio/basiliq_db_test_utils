INSERT INTO articles ("id", "title", "body")
    VALUES ($1, 'Why devs require sacrifices', 'They feast on the blood of the departed draw their powers') RETURNING id;
