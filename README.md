# cogito_api

This is the backend for Cogito, a philosophy research tool envisioned by [William Chastain](https://williamchastain.com)
and powered by this API, written in whole by [Lucas Marta](https://lucasmarta.com).

This API handles user data and connects to the AI service component of Cogito (using gRPC).

This is a HUGE work in progress.

## Setup

### Dependencies
- Cargo (for building and testing `cogito_api`)
- Docker 
### Steps
- Create a `.env` file. (or copy the provided `.env.example` to `.env`)
    ```shell
    cp ./.env.example ./.env
    ```
    It should follow this format. Everything uncommented is required for `cogito_api` to function.
    ```shell
    # ./.env
  
    # Optionally you can have a custom URL/port to host the API on.
    # COGITO_API_URL=127.0.0.1:8080

    # You will need this for docker.
    POSTGRES_USER=postgres
    POSTGRES_PASSWORD=postgres_password
    POSTGRES_DB=cogito_db
  
    # You will need this for the sqlx library to check queries at compile time.
    DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:5432/${POSTGRES_DB}"
    ```
- Start the PostgreSQL database using Docker:
    ```shell
    # Ensure you are in the cogito_api directory.
    docker compose up -d
    ```
- Finally start the API:
    ```shell
    # For now this is the preferred way to run the server. 
    # In the future we can compile binaries or turn this into a docker image.
    cargo run --release
    ```

## OpenAPI

The OpenAPI documentation is available at `/redoc` when the server 
is running. These docs are generated using `utoipa` and `utoipa-redoc`.