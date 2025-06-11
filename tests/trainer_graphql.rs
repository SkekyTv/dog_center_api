use chrono::Utc;
use dog_center_api::infra::db::trainer_repository::PgTrainersRepository;
use dog_center_api::shared::types::sex::Sex;
use dog_center_api::use_cases::trainers_service::CreateTrainerInput;
use dog_center_api::use_cases::trainers_service::TrainersService;
use helpers::app_test::set_up_app_test;
use reqwest::Client;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

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
// query trainer
#[tokio::test]
async fn test_graphql_get_trainer_by_id() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create test data directly using the repository
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);

    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Test Trainer".to_string(),
            sex: Sex::M,
            phone_number: Some("+33987654321".to_string()),
            contact_email: Some("test.trainer@example.com".to_string()),
            birthdate: Some(Utc::now()),
        })
        .await
        .expect("Failed to create test trainer");

    // Query the trainer by ID
    let query = r#"
        query trainer($input: TrainerInput!) {
            trainer(input: $input) {
                id
                name
                sex
                phoneNumber
                contactEmail
                birthdate
            }
        }
    "#;

    let query_variables = json!({
        "input": {
            "id": test_trainer.id.to_string()
        }
    });

    let query_payload = json!({
        "query": query,
        "variables": query_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&query_payload)
        .send()
        .await
        .expect("Failed to send query request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none());

    let trainer = data.get("trainer").expect("Missing `trainer` field");

    assert_eq!(trainer.get("name").unwrap(), "Test Trainer");
    assert_eq!(trainer.get("sex").unwrap(), "M");
    assert_eq!(trainer.get("phoneNumber").unwrap(), "+33987654321");
    assert_eq!(
        trainer.get("contactEmail").unwrap(),
        "test.trainer@example.com"
    );
    assert!(trainer.get("birthdate").is_some());
}

// Test trainer with minimal fields
#[tokio::test]
async fn test_graphql_get_trainer_minimal_fields() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create test data directly using the repository
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);

    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Minimal Trainer".to_string(),
            sex: Sex::F,
            phone_number: None,
            contact_email: None,
            birthdate: None,
        })
        .await
        .expect("Failed to create test trainer");

    let trainer_id = test_trainer.id;

    let query = r#"
        query trainer($input: TrainerInput!) {
            trainer(input: $input) {
                id
                name
                sex
                phoneNumber
                contactEmail
                birthdate
            }
        }
    "#;

    let query_variables = json!({
        "input": {
            "id": trainer_id.to_string()
        }
    });

    let query_payload = json!({
        "query": query,
        "variables": query_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&query_payload)
        .send()
        .await
        .expect("Failed to send query request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let trainer = data.get("trainer").expect("Missing `trainer` field");

    assert_eq!(trainer.get("name").unwrap(), "Minimal Trainer");
    assert_eq!(trainer.get("sex").unwrap(), "F");
    assert!(trainer.get("phoneNumber").unwrap().is_null());
    assert!(trainer.get("contactEmail").unwrap().is_null());
    assert!(trainer.get("birthdate").unwrap().is_null());
}

// Test trainer not found with valid UUID
#[tokio::test]
async fn test_graphql_get_trainer_not_found() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let query = r#"
        query trainer($input: TrainerInput!) {
            trainer(input: $input) {
                id
                name
            }
        }
    "#;

    // Use a valid UUID format that doesn't exist
    let non_existent_id = Uuid::new_v4();
    let query_variables = json!({
        "input": {
            "id": non_existent_id.to_string()
        }
    });

    let query_payload = json!({
        "query": query,
        "variables": query_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&query_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let errors = response_json
        .get("errors")
        .expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert_eq!(message, "Trainer not found");
}

// Test trainer query with invalid UUID format
#[tokio::test]
async fn test_graphql_get_trainer_invalid_uuid() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let query = r#"
        query trainer($input: TrainerInput!) {
            trainer(input: $input) {
                id
                name
            }
        }
    "#;

    let query_variables = json!({
        "input": {
            "id": "invalid-uuid-format"
        }
    });

    let query_payload = json!({
        "query": query,
        "variables": query_variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&query_payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let errors = response_json
        .get("errors")
        .expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert!(
        message.contains("UUID") || message.contains("invalid") || message.contains("format"),
        "Expected UUID validation error, got: {}",
        message
    );
}
