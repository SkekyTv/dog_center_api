use helpers::app_test::set_up_app_test;
use reqwest::Client;
use serde_json::{Value, json};

mod helpers;

#[tokio::test]
async fn test_graphql_sign_up_success() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let mutation = r#"
        mutation signUp($input: SignUpInput!) {
            signUp(input: $input) {
                id
                email
            }
        }
    "#;

    let variables = json!({
        "input": {
            "email": "user1@example.com",
            "pdw": "password123"
        }
    });

    let payload = json!({
        "query": mutation,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none(), "Unexpected errors: {:?}", errors);

    let user = data.get("signUp").expect("Missing `signUp` field");
    assert_eq!(user.get("email").unwrap(), "user1@example.com");
    assert!(user.get("id").is_some());
}

#[tokio::test]
async fn test_graphql_sign_up_duplicate_email() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // First sign up
    let mutation = r#"
        mutation signUp($input: SignUpInput!) {
            signUp(input: $input) {
                id
                email
            }
        }
    "#;

    let variables = json!({
        "input": {
            "email": "user2@example.com",
            "pdw": "password123"
        }
    });

    let payload = json!({
        "query": mutation,
        "variables": variables
    });

    let _ = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    // Second sign up with same email
    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let errors = response_json
        .get("errors")
        .expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert!(
        message.contains("already") || message.contains("exists"),
        "Expected duplicate email error, got: {}",
        message
    );
}

#[tokio::test]
async fn test_graphql_login_success() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // First, sign up the user
    let sign_up_mutation = r#"
        mutation signUp($input: SignUpInput!) {
            signUp(input: $input) {
                id
                email
            }
        }
    "#;

    let sign_up_variables = json!({
        "input": {
            "email": "user3@example.com",
            "pdw": "password123"
        }
    });

    let sign_up_payload = json!({
        "query": sign_up_mutation,
        "variables": sign_up_variables
    });

    let _ = client
        .post(format!("{}/graphql", app_url))
        .json(&sign_up_payload)
        .send()
        .await
        .expect("Failed to send sign up request");

    // Now, login
    let login_mutation = r#"
        mutation login($input: LoginInput!) {
            login(input: $input) {
                token
            }
        }
    "#;

    let login_variables = json!({
        "input": {
            "email": "user3@example.com",
            "pdw": "password123"
        }
    });

    let login_payload = json!({
        "query": login_mutation,
        "variables": login_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to send login request");

    assert!(response.status().is_success());

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none(), "Unexpected errors: {:?}", errors);

    let login = data.get("login").expect("Missing `login` field");
    assert!(login.get("token").is_some());
}

#[tokio::test]
async fn test_graphql_login_wrong_password() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // First, sign up the user
    let sign_up_mutation = r#"
        mutation signUp($input: SignUpInput!) {
            signUp(input: $input) {
                id
                email
            }
        }
    "#;

    let sign_up_variables = json!({
        "input": {
            "email": "user4@example.com",
            "pdw": "password123"
        }
    });

    let sign_up_payload = json!({
        "query": sign_up_mutation,
        "variables": sign_up_variables
    });

    let _ = client
        .post(format!("{}/graphql", app_url))
        .json(&sign_up_payload)
        .send()
        .await
        .expect("Failed to send sign up request");

    // Now, try to login with wrong password
    let login_mutation = r#"
        mutation login($input: LoginInput!) {
            login(input: $input) {
                token
            }
        }
    "#;

    let login_variables = json!({
        "input": {
            "email": "user4@example.com",
            "pdw": "wrongpassword"
        }
    });

    let login_payload = json!({
        "query": login_mutation,
        "variables": login_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to send login request");

    assert!(response.status().is_success());

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let errors = response_json
        .get("errors")
        .expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert!(
        message.contains("login")
            || message.contains("credentials")
            || message.contains("password"),
        "Expected login error, got: {}",
        message
    );
}
