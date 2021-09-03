# Notes lors de ce projet

## Diesel

une fois que le .env à été créé, on peut faire `diesel setup` pour metre en place la BDD.

Migrations : 
```bash
# create
diesel migration generate create_contraptions

# run
diesel migration run

#redo

diesel migration redo
```