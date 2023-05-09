curl --location --request POST 'localhost:3000/api/categories' \
--header 'Authorization: Basic YWRtaW46aGVsbG9fcnVzdA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Horror",
    "parent_id": 1
}'