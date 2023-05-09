curl --location --request POST 'localhost:3000/api/stories' \
--header 'Authorization: Basic c3RhbjpwYXJhbm9pZA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "ADA is up more than 1000%",
    "content": "Today at the world of blockchain we celebrate a big day.",
    "category_id": 6,
    "tags": [
        1,
        3
    ]
}'