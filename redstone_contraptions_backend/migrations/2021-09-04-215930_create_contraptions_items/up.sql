CREATE TABLE ContraptionsItems (
    contraptions_id INTEGER REFERENCES Contraptions (id),
    items_id INTEGER REFERENCES Items (id),
    CONSTRAINT contraptions_items_pkey PRIMARY KEY (contraptions_id, items_id)
);