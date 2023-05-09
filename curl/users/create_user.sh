curl --location --request POST 'localhost:3000/api/users' \
--header 'Authorization: Basic YWRtaW46aGVsbG9fcnVzdA==' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Stanislav",
    "login": "stan",
    "password": "paranoid",
    "is_admin": false,
    "is_author": true
}'