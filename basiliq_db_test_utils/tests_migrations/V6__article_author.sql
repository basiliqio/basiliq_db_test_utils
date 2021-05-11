CREATE TABLE "people-article"(
	people_id		UUID NOT NULL REFERENCES peoples ON DELETE CASCADE,
	article_id		UUID NOT NULL REFERENCES articles ON DELETE CASCADE,
	PRIMARY KEY (people_id, article_id)
);
