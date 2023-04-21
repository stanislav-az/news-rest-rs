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

## Auth

[Basic](https://en.wikipedia.org/wiki/Basic_access_authentication) access authentication is implemented for this project.
In short: there is a header included inside each guarded request. This header is looking like this `Authorization: Basic *`,
where `*` is login and password concatenated with `:` in the middle (`login:password`) and base 64 encoded.

### Admin

Administrator user is created inside migrations (insert_superuser). His login is "admin" and his password is "hello_rust".
You can encode it for using inside header using postman, or in linux terminal:

```sh
echo -n "admin:hello_rust" | base64
```
