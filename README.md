# cogito_api

This is the backend for Cogito, a philosophy research tool.

This API handles user data and connects to the AI service component of Cogito (using gRPC).

This is a HUGE work in progress.

## Setup

- Create a `.env` file.
- Put your database URL inside under the DATABASE_URL variable:
    ```shell
    DATABASE_URL=postgres://user:pass@127.0.0.1:5432/dbname
  
    # Optionally you can have a custom URL/port.
    # COGITO_API_URL=127.0.0.1:8080
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

## OpenAPI

The OpenAPI documentation is available at `/redoc` when the server 
is running. These docs are generated using `utoipa` and `utoipa-redoc`.