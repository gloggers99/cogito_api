# cogito_api

This is the backend for Cogito, a philosophy research tool.

This API handles user data and connects to the AI service component of Cogito (using gRPC).

This is a HUGE work in progress.

## Setup

- Create a `.env` file.
- Put your database URL inside under the DATABASE_URL variable:
    ```shell
    DATABASE_URL=postgres://user:pass@127.0.0.1:5432/dbname
    ```
- Run the `init.sql` file on your database.

## Quick PostgreSQL tutorial

```shell
# Create database for cogito
createdb -U postgres cogito_db

# Now you can access the command prompt for the server
psql -U postgres -d cogito_db

# Or directly run the script
psql -U postgres -d cogito_db -f "init.sql"
```

## Endpoints
- `/login`
    - Expects a json form in the format of:
      ```json
      {
        "username": "john",
        "password": "abc123"
      }
      ```
      If the credentials match, the server will respond with a cookie with your access UUID.
- `/user/{id}`
    - Retrieve public user information in json form.