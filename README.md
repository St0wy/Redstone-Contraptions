# Redstone Contraptions

## Backend setup

1. Install PostgreSQL
2. Install Diesel CLI
```bash
cargo install diesel_cli
```
3. Create a .env file
```.env
DATABASE_URL=postgres://postgres:password@localhost/redstone_contraptions
```
4. Run `diesel setup`