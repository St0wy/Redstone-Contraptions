CREATE TABLE Contraptions (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    image TEXT,
    itemsList TEXT
)