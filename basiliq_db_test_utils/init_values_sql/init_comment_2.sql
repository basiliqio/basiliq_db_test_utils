INSERT INTO comments ("id", "author", "article", "body")
    VALUES ('6ae9938f-d490-4707-b138-770c2a52465f', $1, $2, 'It was great !') RETURNING id;
