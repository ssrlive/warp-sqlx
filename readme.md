# warp-sqlx

A example app to provide API from database.

To run this app, you need to install [Rust](https://www.rust-lang.org/tools/install) and [PostgreSQL](https://www.postgresql.org/download/).

Please login to PostgreSQL with `postgres` account and set password `postgres` for it. In Linux, you can run the following commands.

```bash
sudo -i -u postgres
psql
\password
\q
```

Then, run the following commands to play this app.

```bash
cargo run
```
