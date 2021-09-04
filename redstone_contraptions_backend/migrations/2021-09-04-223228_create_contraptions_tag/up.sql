CREATE TABLE ContraptionsTags (
    contraptions_id INTEGER REFERENCES Contraptions (id),
    tags_id INTEGER REFERENCES Items (id),
    CONSTRAINT contraptions_tags_pkey PRIMARY KEY (contraptions_id, tags_id)
);