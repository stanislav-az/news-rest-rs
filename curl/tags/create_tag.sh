curl --location --request POST 'localhost:3000/api/tags' \
--header 'Authorization: Basic c3RhbjpwYXJhbm9pZA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "cryptocurrency"
}'