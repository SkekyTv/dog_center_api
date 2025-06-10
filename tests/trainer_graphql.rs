use helpers::app_test::set_up_app_test;
use reqwest::Client;
use serde_json::Value;
use serde_json::json;

mod helpers;

// registerTrainer with all fields
#[tokio::test]
async fn test_graphql_register_trainer() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();

    let client = Client::new();

    // Register Trainer
    let mutation = r#"
        mutation registerTrainer($input: RegisterTrainerInput!) {
            registerTrainer(input: $input) {
                id
                name
                sex
                phoneNumber
                contactEmail
                birthdate
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "John Doe",
            "sex": "M",
            "phoneNumber": "+33123456789",
            "contactEmail": "john.doe@example.com",
            "birthdate": "2023-01-15T10:30:00Z"
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

    assert!(
        response.status().is_success(),
        "Response status is not success"
    );

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    match errors {
        Some(e) => println!("errors: {}", e),
        None => println!("no errors"),
    }

    let register_trainer = data
        .get("registerTrainer")
        .expect("Missing `registerTrainer` field");

    assert_eq!(register_trainer.get("name").unwrap(), "John Doe");
    assert_eq!(register_trainer.get("sex").unwrap(), "M");
    assert_eq!(register_trainer.get("phoneNumber").unwrap(), "+33123456789");
    assert_eq!(
        register_trainer.get("contactEmail").unwrap(),
        "john.doe@example.com"
    );
    assert!(register_trainer.get("id").is_some());
    assert!(register_trainer.get("birthdate").is_some());
}

// registerTrainer with minimal required fields
#[tokio::test]
async fn test_graphql_register_trainer_minimal() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();

    let client = Client::new();

    let mutation = r#"
        mutation registerTrainer($input: RegisterTrainerInput!) {
            registerTrainer(input: $input) {
                id
                name
                sex
                phoneNumber
                contactEmail
                birthdate
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "Jane Smith",
            "sex": "F"
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

    assert!(errors.is_none());

    let register_trainer = data
        .get("registerTrainer")
        .expect("Missing `registerTrainer` field");

    assert_eq!(register_trainer.get("name").unwrap(), "Jane Smith");
    assert_eq!(register_trainer.get("sex").unwrap(), "F");
    assert!(register_trainer.get("phoneNumber").unwrap().is_null());
    assert!(register_trainer.get("contactEmail").unwrap().is_null());
    assert!(register_trainer.get("birthdate").unwrap().is_null());
    assert!(register_trainer.get("id").is_some());
}

// registerTrainer with invalid input (empty name)
#[tokio::test]
async fn test_graphql_register_trainer_invalid_input() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();

    let client = Client::new();

    let mutation = r#"
        mutation registerTrainer($input: RegisterTrainerInput!) {
            registerTrainer(input: $input) {
                id
                name
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "",
            "sex": "M"
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

    let body: Value = response.json().await.expect("Invalid JSON response");

    let errors = body.get("errors").expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert!(
        message.contains("name") || message.contains("empty") || message.contains("invalid"),
        "Expected validation error for empty name, got: {}",
        message
    );
}
