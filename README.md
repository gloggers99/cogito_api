# Philosopapi (philosophy api)

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