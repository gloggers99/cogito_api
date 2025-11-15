#[cfg(test)]
mod tests {
    use reqwest::{Client, ClientBuilder};
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
    struct RegisterResponse {
        message: String,
    }

    #[derive(Serialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Deserialize)]
    struct LoginResponse {
        message: String,
    }

    /// Test registration, login, and session-protected route
    #[tokio::test]
    async fn test_register_login_and_access_protected_route() {
        // Enable cookie store so session is preserved
        let client = ClientBuilder::cookie_store(Client::builder(), true)
            .build()
            .unwrap();

        let register_url = "http://127.0.0.1:8080/register";
        let login_url = "http://127.0.0.1:8080/login";
        let protected_url = "http://127.0.0.1:8080/users/1";

        // Use a unique username/email for the test to avoid conflicts
        let unique_id = Uuid::new_v4().to_string();
        let register_req = RegisterRequest {
            email: format!("twin+{}@example.com", unique_id),
            phone_number: format!("555-1234-{}", &unique_id[..6]),
            username: format!("twin_test_{}", &unique_id[..6]),
            password: "sherm".into(),
        };

        // 1️⃣ Register the account
        let register_resp = client
            .post(register_url)
            .json(&register_req)
            .send()
            .await
            .expect("Failed to send register request");

        assert!(
            register_resp.status().is_success(),
            "Registration failed: {}",
            register_resp.text().await.unwrap()
        );

        let register_body: RegisterResponse = register_resp
            .json()
            .await
            .expect("Failed to deserialize register response");

        println!("Register message: {}", register_body.message);

        // 2️⃣ Login with the same credentials
        let login_req = LoginRequest {
            username: register_req.username.clone(),
            password: register_req.password.clone(),
        };

        let login_resp = client
            .post(login_url)
            .json(&login_req)
            .send()
            .await
            .expect("Failed to send login request");

        assert!(login_resp.status().is_success(), "Login failed");

        let login_body: LoginResponse = login_resp
            .json()
            .await
            .expect("Failed to deserialize login response");

        println!("Login message: {}", login_body.message);

        // 3️⃣ Access protected route
        let protected_resp = client
            .get(protected_url)
            .send()
            .await
            .expect("Failed to send request to protected endpoint");

        assert!(
            protected_resp.status().is_success(),
            "Access denied to protected endpoint"
        );

        let content = protected_resp
            .text()
            .await
            .expect("Failed to read response body");

        println!("Protected endpoint response: {}", content);
    }
}
