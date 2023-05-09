curl --location --request PATCH 'localhost:3000/api/categories/1' \
--header 'Authorization: Basic YWRtaW46aGVsbG9fcnVzdA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Film",
    "parent_id": 6
}'