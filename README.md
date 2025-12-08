# cogito_api

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/c9e12ce76ba5439a99fe03880d2180db)](https://app.codacy.com/gh/gloggers99/cogito_api/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

This is the backend for Cogito, a Q&A style agentic philosophy research tool envisioned by [William Chastain](https://www.williamchastain.com)
and powered by this API, written in whole by [Lucas Marta](https://lucasmarta.com).

This API handles user data and connects to the AI service component of Cogito ([CrazyWillBear/cogito-ai](https://github.com/CrazyWillBear/cogito-ai)) using gRPC.

This is a HUGE work in progress.

## Setup

### Dependencies

- Cargo (for building and testing `cogito_api`)
- Docker 
- Protocol Buffer Compiler (`protoc`)

### Steps

- Create a `.env` file. (or copy the provided `.env.example` to `.env`)
    ```shell
    cp ./.env.example ./.env
    ```
    It should follow this format. Everything uncommented is required for `cogito_api` to function.
    ```shell
    # ./.env
      
    # Database credentials.
    POSTGRES_USER=postgres
    POSTGRES_PASSWORD=password
    POSTGRES_DB=cogito_db
    
    # Optional IP & port to bind the API service to.
    # If this does not exist the default will be "127.0.0.1:8080"
    COGITO_API_URL=127.0.0.1:8080
    
    # The IP & port of the Cogito agent.
    COGITO_AGENT_URL=127.0.0.1:9999
    ```
- Finally start the API:
    ```shell
    # This will start both the api and the database.
    docker compose up -d
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
