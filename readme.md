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

Then, run the following commands to play this toy.

```bash
cargo run
```

Use [curl](https://curl.se/) to test it.

```bash
# create a product
curl http://localhost:3030/api/products/ -X POST -H 'Content-Type: application/json' -d '{"name":"John"}'

# create another product
curl http://localhost:3030/api/products/ -X POST -H 'Content-Type: application/json' -d '{"name":"Peter"}'

# get all products
curl http://localhost:3030/api/products/

# get a product
curl http://localhost:3030/api/products/1

# update a product
curl http://localhost:3030/api/products/1 -X PUT -H 'Content-Type: application/json' -d '{"name":"Adam"}'

# delete a product
curl http://localhost:3030/api/products/2 -X DELETE

```
