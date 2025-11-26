# cogito_api

This is the backend for Cogito, a Q&A style agentic philosophy research tool envisioned by [William Chastain](https://www.williamchastain.com)
and powered by this API, written in whole by [Lucas Marta](https://lucasmarta.com).

This API handles user data and connects to the AI service component of Cogito ([CrazyWillBear/cogito-ai](https://github.com/CrazyWillBear/cogito-ai)) using gRPC.

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
    
    # The URL for the Cogito agent's gRPC server.
    COGITO_AGENT_URL="127.0.0.1:9999"
    
    # This is for `.proto` compiled files.
    OUT_DIR="src/generated"
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

The OpenAPI documentation is available at `/redoc` when the server is running. These docs are generated using the 
`utoipa` and `utoipa-redoc` rust crates.

## Additional Information
This repository is fully implemented by me and serves as an opportunity to demonstrate the range of technologies and 
practices I'm working to master.  

The AI agent component that this API interacts with is being developed independently by Will 
[here](https://github.com/CrazyWillBear/research-langgraph). 

Primary technologies demonstrated in this repository:

- Rust
- PostgreSQL
- Docker
- gRPC
- OpenAPI
