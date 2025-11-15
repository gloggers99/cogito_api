#[cfg(test)]
mod tests {
    use reqwest::{Client, ClientBuilder};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Deserialize)]
    struct LoginResponse {
        message: String,
    }

    /// Test the login system & the session verification system.
    ///
    /// This test was written by ChatGPT.
    #[tokio::test]
    async fn test_login_and_access_protected_route() {
        let client = ClientBuilder::cookie_store(Client::builder(), true) // <-- stores cookies automatically
            .build()
            .unwrap();

        let login_url = "http://127.0.0.1:8080/login";
        let protected_url = "http://127.0.0.1:8080/users/1";

        let login_req = LoginRequest {
            username: "mike".into(),
            password: "sherm".into(),
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

        let protected_resp = client
            .get(protected_url)
            .send()
            .await
            .expect("Failed to send request to protected endpoint");

        assert!(protected_resp.status().is_success(), "Access denied to protected endpoint");

        let content = protected_resp.text().await.expect("Failed to read response body");

        println!("Protected endpoint response: {}", content);
    }
}
