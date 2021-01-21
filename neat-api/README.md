# Tutorial: Creating a rest api in rust with warp

Following the tutorial here: https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/

## Curls for testing server
Post
```
curl --location --request POST 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple",
    "quantity": 3
}'
```

Update
```
curl --location --request PUT 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple",
    "quantity": 5
}'
```


Get
```
curl --location --request GET 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'
```


Delete
```
curl --location --request DELETE 'localhost:3030/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple"
}'
```