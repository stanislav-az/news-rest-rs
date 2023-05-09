# news-rest-rs

A simple blog server in rust using axum and diesel. Run server with `cargo run`. It will start at `http://localhost:3000`.

## Environment configuration

Start with a template:

```sh
cp template.env .env
```

And modify env variables there.

P.S. You can generate `SALT_16_BYTES_BASE_64` var using `openssl rand -base64 16`.

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

## :gear: Functionality (relative URLs)

### GET api/stories

* Supports pagination. To receive paginated data provide query parameters of limit and offset. Example: ?limit=15&offset=0.
* Supports filtering:
  - By date created. Get news posted at exact date with ?creation_date_at, or posted earlier with ?creation_date_until, or posted later with ?creation_date_since.
  - By author name (exact match) with ?author_name.
  - By category id with ?category_id.
  - By tags. Get posts including some tag in a list with ?tag_in.
  - By phrase in title (case ignored ILIKE regex) with ?title_ilike.
  - By phase in content (case ignored ILIKE regex) with ?content_ilike.
* Supports sorting with ?sort_by:
  - By author name ?sort_by=author.
  - By category name ?sort_by=category.
  - By date created ascending ?sort_by=creation_timestamp_asc.
  - By date created descending ?sort_by=creation_timestamp_desc.

### GET api/stories/search/{yoursearchqueryhere}

* Finds search query in post title/content or in a author/tag/category name matching ILIKE regex.
* Supports pagination. To receive paginated data provide query parameters of limit and offset. Example: ?limit=15&offset=0.
* Supports sorting with ?sort_by:
  - By author name ?sort_by=author.
  - By category name ?sort_by=category.
  - By date created ascending ?sort_by=creation_timestamp_asc.
  - By date created descending ?sort_by=creation_timestamp_desc.

### GET methods

* api/stories/{id} :id:
* api/users
* api/tags
* api/categories

### POST methods

* api/stories :id:
* api/users :no_entry_sign:
* api/tags :id:
* api/categories :no_entry_sign:

### Publish news

* POST api/stories/{id} :id:

### PATCH methods

* api/stories/{id} :id:
* api/users/{id} :id:
* api/categories/{id} :no_entry_sign:

### DELETE methods

* api/stories/{id} :id: or :no_entry_sign:
* api/users/{id} :id: or :no_entry_sign:
* api/tags/{id} :no_entry_sign:
* api/categories/{id} :no_entry_sign:

### Authorization

* Paths which require admin authorization are marked by :no_entry_sign:.
* Paths which require author authorization are marked by :id:.

## :ledger: Logging

Supported levels of logging are: Trace, Debug, Info, Warn, Error. Configure log filters with `RUST_LOG` env var.
