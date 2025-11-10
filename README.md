# cogito_api

This is the backend for Cogito, a philosophy research tool.

This API handles user data and connects to the AI service component of Cogito (using gRPC).

This is a HUGE work in progress.

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