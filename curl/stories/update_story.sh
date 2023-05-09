curl --location --request PATCH 'localhost:3000/api/stories/2' \
--header 'Authorization: Basic c3RhbjpwYXJhbm9pZA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "ADA is up more than 10%",
    "content": "Today at the world of blockchain we celebrate a very big day."
}'