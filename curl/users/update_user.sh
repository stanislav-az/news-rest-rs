curl --location --request PATCH 'localhost:3000/api/users/10' \
--header 'Authorization: Basic c3RhbjpwYXJhbm9pZA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Stan"
}'