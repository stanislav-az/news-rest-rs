# news-rest-rs

A simple blog server in rust using axum and diesel.

## Environment configuration

Start with a template:

```sh
cp template.env .env
```

And modify all env variables there.
P.S. You can generate SALT_16_BYTES_BASE_64 var using `openssl rand -base64 16`.

## Diesel database management

If this tool is not installed, run `cargo install diesel_cli --no-default-features --features postgres`.

### Initialization

If you don't have a user with database creation permission - connect to postgres and create him:

```SQL
CREATE USER news_rs_admin WITH PASSWORD 'your_password' CREATEDB;
```

Then run `diesel setup`.

### Connecting through psql

```sh
psql -U news_rs_admin -d news_rs -p 5432 -h 127.0.0.1
```

### Generating new migrations

```sh
diesel migration generate description_of_the_change
```

### Running migrations

```sh
diesel migration run
```
