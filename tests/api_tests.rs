#[cfg(test)]
mod tests {
    use reqwest::{Client, ClientBuilder, StatusCode};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize)]
    struct RegisterRequest {
        email: String,
        phone_number: String,
        username: String,
        password: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct RegisterResponse {
        message: String,
    }

    #[derive(Serialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct LoginResponse {
        message: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct GenericResponse {
        message: String,
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct User {
        user_id: i32,
        user_email: String,
        user_phone: String,
        user_name: String,
        user_pass: String,
        user_last_login: String,
        login_id: Option<serde_json::Value>,
        verified: bool,
        admin: bool,
    }

    const BASE_URL: &str = "http://127.0.0.1:8080";

    /// Helper function to create a client with cookie support
    fn create_client() -> Client {
        ClientBuilder::cookie_store(Client::builder(), true)
            .build()
            .unwrap()
    }

    /// Helper function to login a user
    async fn login_user(client: &Client, username: &str, password: &str) -> reqwest::Response {
        let login_req = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        client
            .post(format!("{}/login", BASE_URL))
            .json(&login_req)
            .send()
            .await
            .expect("Failed to send login request")
    }

    /// Test that login fails with invalid credentials
    #[tokio::test]
    async fn test_login_with_invalid_credentials() {
        let client = create_client();

        let resp = login_user(&client, "nonexistent_user_12345", "wrongpassword").await;

        assert_eq!(
            resp.status(),
            StatusCode::FORBIDDEN,
            "Expected 403 Forbidden for invalid credentials"
        );

        let body: GenericResponse = resp.json().await.expect("Failed to parse response");
        assert_eq!(body.message, "Invalid credentials.");
    }

    /// Test that accessing protected routes without authentication fails
    #[tokio::test]
    async fn test_unauthenticated_access_to_protected_route() {
        let client = create_client();

        let resp = client
            .get(format!("{}/users/1", BASE_URL))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            resp.status(),
            StatusCode::UNAUTHORIZED,
            "Expected 401 Unauthorized for unauthenticated request"
        );
    }

    /// Test registration with duplicate username fails
    #[tokio::test]
    async fn test_duplicate_registration() {
        let client = create_client();
        let unique_id = Uuid::new_v4().to_string();
        let username = format!("dup_test_{}", &unique_id[..6]);

        let register_req = RegisterRequest {
            email: format!("dup_test_{}@example.com", unique_id),
            phone_number: format!("555-dup-{}", &unique_id[..6]),
            username: username.clone(),
            password: "testpass".to_string(),
        };

        // First registration should succeed
        let resp1 = client
            .post(format!("{}/register", BASE_URL))
            .json(&register_req)
            .send()
            .await
            .expect("Failed to send first register request");

        assert!(
            resp1.status().is_success(),
            "First registration should succeed"
        );

        // Second registration with the same username should fail
        let resp2 = client
            .post(format!("{}/register", BASE_URL))
            .json(&register_req)
            .send()
            .await
            .expect("Failed to send second register request");

        assert_eq!(
            resp2.status(),
            StatusCode::CONFLICT,
            "Expected 409 Conflict for duplicate registration"
        );

        let body: RegisterResponse = resp2.json().await.expect("Failed to parse response");
        assert!(
            body.message.contains("already exists"),
            "Error message should indicate duplicate"
        );
    }

    /// Test that login with wrong password fails
    #[tokio::test]
    async fn test_login_with_wrong_password() {
        let client = create_client();
        let unique_id = Uuid::new_v4().to_string();
        let username = format!("wrongpw_test_{}", &unique_id[..6]);
        let correct_password = "correctpassword";
        let wrong_password = "wrongpassword";

        // Register a new user
        let register_req = RegisterRequest {
            email: format!("wrongpw_{}@example.com", unique_id),
            phone_number: format!("555-wpw-{}", &unique_id[..6]),
            username: username.clone(),
            password: correct_password.to_string(),
        };

        let reg_resp = client
            .post(format!("{}/register", BASE_URL))
            .json(&register_req)
            .send()
            .await
            .expect("Failed to register user");

        assert!(
            reg_resp.status().is_success(),
            "Registration should succeed"
        );

        // Try to login with wrong password
        let resp = login_user(&client, &username, wrong_password).await;

        assert_eq!(
            resp.status(),
            StatusCode::FORBIDDEN,
            "Expected 403 Forbidden for wrong password"
        );

        let body: GenericResponse = resp.json().await.expect("Failed to parse response");
        assert_eq!(body.message, "Invalid credentials.");
    }

    /// Test retrieving user info after authentication
    #[tokio::test]
    async fn test_get_user_after_login() {
        let client = create_client();
        let unique_id = Uuid::new_v4().to_string();
        let username = format!("getuser_test_{}", &unique_id[..6]);
        let password = "testpassword";

        // Register and login
        let register_req = RegisterRequest {
            email: format!("getuser_{}@example.com", unique_id),
            phone_number: format!("555-gu-{}", &unique_id[..6]),
            username: username.clone(),
            password: password.to_string(),
        };

        let reg_resp = client
            .post(format!("{}/register", BASE_URL))
            .json(&register_req)
            .send()
            .await
            .expect("Failed to register user");

        assert!(
            reg_resp.status().is_success(),
            "Registration should succeed"
        );

        // Login
        let login_resp = login_user(&client, &username, password).await;
        assert!(login_resp.status().is_success(), "Login should succeed");

        // Get user by ID 1 (assuming there's at least one user)
        let user_resp = client
            .get(format!("{}/users/1", BASE_URL))
            .send()
            .await
            .expect("Failed to get user");

        assert!(
            user_resp.status().is_success(),
            "Should be able to access user endpoint after login"
        );

        let user: User = user_resp
            .json()
            .await
            .expect("Failed to parse user response");
        assert_eq!(user.user_id, 1, "User ID should match");
    }

    /// Test accessing non-existent user returns 404
    #[tokio::test]
    async fn test_get_nonexistent_user() {
        let client = create_client();
        let unique_id = Uuid::new_v4().to_string();
        let username = format!("nouser_test_{}", &unique_id[..6]);
        let password = "testpassword";

        // Register and login first
        let register_req = RegisterRequest {
            email: format!("nouser_{}@example.com", unique_id),
            phone_number: format!("555-nu-{}", &unique_id[..6]),
            username: username.clone(),
            password: password.to_string(),
        };

        let reg_resp = client
            .post(format!("{}/register", BASE_URL))
            .json(&register_req)
            .send()
            .await
            .expect("Failed to register user");

        assert!(
            reg_resp.status().is_success(),
            "Registration should succeed"
        );

        let login_resp = login_user(&client, &username, password).await;
        assert!(login_resp.status().is_success(), "Login should succeed");

        // Try to get a non-existent user (very high ID)
        let user_resp = client
            .get(format!("{}/users/99999999", BASE_URL))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            user_resp.status(),
            StatusCode::NOT_FOUND,
            "Expected 404 Not Found for non-existent user"
        );

        let body: GenericResponse = user_resp.json().await.expect("Failed to parse response");
        assert_eq!(body.message, "User not found.");
    }
}
