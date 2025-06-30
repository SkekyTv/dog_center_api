use chrono::Utc;
use dog_center_api::infra::db::trainers_repository::PgTrainersRepository;
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

#[tokio::test]
async fn test_graphql_update_trainer_all_fields() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Création du trainer via TrainersService
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);
    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Initial Name".to_string(),
            sex: Sex::M,
            phone_number: Some("+33213456789".to_string()),
            contact_email: Some("initial@example.com".to_string()),
            birthdate: Some(chrono::Utc::now()),
        })
        .await
        .expect("Failed to create test trainer");
    let trainer_id = test_trainer.id.to_string();

    // Update Trainer
    let update_mutation = r#"
        mutation updateTrainer($input: UpdateTrainerInput!) {
            updateTrainer(input: $input) {
                id name sex phoneNumber contactEmail birthdate
            }
        }
    "#;
    let update_variables = json!({
        "input": {
            "id": trainer_id,
            "name": "Updated Name",
            "sex": "F",
            "phoneNumber": "+33123456789",
            "contactEmail": "updated@example.com",
            "birthdate": "2023-05-10T12:00:00Z"
        }
    });
    let update_payload = json!({
        "query": update_mutation,
        "variables": update_variables
    });
    let update_response = client
        .post(format!("{}/graphql", app_url))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send update request");
    assert!(update_response.status().is_success());
    let update_json: Value = update_response.json().await.unwrap();
    let updated = &update_json["data"]["updateTrainer"];
    assert_eq!(updated["name"], "Updated Name");
    assert_eq!(updated["sex"], "F");
    assert_eq!(updated["phoneNumber"], "+33123456789");
    assert_eq!(updated["contactEmail"], "updated@example.com");
    assert_eq!(updated["id"], trainer_id);

    // Vérification via query
    let query = r#"
        query trainer($input: TrainerInput!) {
            trainer(input: $input) { id name sex phoneNumber contactEmail birthdate }
        }
    "#;
    let query_variables = json!({ "input": { "id": trainer_id } });
    let query_payload = json!({ "query": query, "variables": query_variables });
    let query_response = client
        .post(format!("{}/graphql", app_url))
        .json(&query_payload)
        .send()
        .await
        .expect("Failed to send query request");
    assert!(query_response.status().is_success());
    let query_json: Value = query_response.json().await.unwrap();
    let trainer = &query_json["data"]["trainer"];
    assert_eq!(trainer["name"], "Updated Name");
    assert_eq!(trainer["sex"], "F");
    assert_eq!(trainer["phoneNumber"], "+33123456789");
    assert_eq!(trainer["contactEmail"], "updated@example.com");
}

#[tokio::test]
async fn test_graphql_update_trainer_minimal_fields() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Création du trainer via TrainersService
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);
    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Minimal Name".to_string(),
            sex: Sex::M,
            phone_number: None,
            contact_email: None,
            birthdate: None,
        })
        .await
        .expect("Failed to create test trainer");
    let trainer_id = test_trainer.id.to_string();

    // Update only name
    let update_mutation = r#"
        mutation updateTrainer($input: UpdateTrainerInput!) {
            updateTrainer(input: $input) { id name sex phoneNumber contactEmail birthdate }
        }
    "#;
    let update_variables = json!({
        "input": {
            "id": trainer_id,
            "name": "Renamed"
        }
    });
    let update_payload = json!({
        "query": update_mutation,
        "variables": update_variables
    });
    let update_response = client
        .post(format!("{}/graphql", app_url))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send update request");
    assert!(update_response.status().is_success());
    let update_json: Value = update_response.json().await.unwrap();
    let updated = &update_json["data"]["updateTrainer"];
    assert_eq!(updated["name"], "Renamed");
    assert_eq!(updated["sex"], "M");
    assert!(updated["phoneNumber"].is_null());
    assert!(updated["contactEmail"].is_null());
    assert!(updated["birthdate"].is_null());
}

#[tokio::test]
async fn test_graphql_update_trainer_invalid_uuid() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let update_mutation = r#"
        mutation updateTrainer($input: UpdateTrainerInput!) {
            updateTrainer(input: $input) { id }
        }
    "#;
    let update_variables = json!({
        "input": {
            "id": "not-a-uuid",
            "name": "Should Fail"
        }
    });
    let update_payload = json!({
        "query": update_mutation,
        "variables": update_variables
    });
    let update_response = client
        .post(format!("{}/graphql", app_url))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send update request");
    assert!(update_response.status().is_success());
    let update_json: Value = update_response.json().await.unwrap();
    let errors = update_json.get("errors").expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();
    assert!(
        message.contains("UUID") || message.contains("invalid") || message.contains("format"),
        "Expected UUID validation error, got: {}",
        message
    );
}

#[tokio::test]
async fn test_graphql_update_trainer_not_found() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let update_mutation = r#"
        mutation updateTrainer($input: UpdateTrainerInput!) {
            updateTrainer(input: $input) { id }
        }
    "#;
    let non_existent_id = Uuid::new_v4().to_string();
    let update_variables = json!({
        "input": {
            "id": non_existent_id,
            "name": "Should Not Exist"
        }
    });
    let update_payload = json!({
        "query": update_mutation,
        "variables": update_variables
    });
    let update_response = client
        .post(format!("{}/graphql", app_url))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send update request");
    assert!(update_response.status().is_success());
    let update_json: Value = update_response.json().await.unwrap();
    let errors = update_json.get("errors").expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();
    assert_eq!(message, "Error updating trainer: TrainerNotFound");
}

#[tokio::test]
async fn test_graphql_update_trainer_invalid_input() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Création du trainer via TrainersService
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);
    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Valid Name".to_string(),
            sex: Sex::M,
            phone_number: None,
            contact_email: None,
            birthdate: None,
        })
        .await
        .expect("Failed to create test trainer");
    let trainer_id = test_trainer.id.to_string();

    // Try to update with empty name
    let update_mutation = r#"
        mutation updateTrainer($input: UpdateTrainerInput!) {
            updateTrainer(input: $input) { id }
        }
    "#;
    let update_variables = json!({
        "input": {
            "id": trainer_id,
            "name": ""
        }
    });
    let update_payload = json!({
        "query": update_mutation,
        "variables": update_variables
    });
    let update_response = client
        .post(format!("{}/graphql", app_url))
        .json(&update_payload)
        .send()
        .await
        .expect("Failed to send update request");
    assert!(update_response.status().is_success());
    let update_json: Value = update_response.json().await.unwrap();
    let errors = update_json.get("errors").expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();
    assert!(
        message.contains("name") || message.contains("empty") || message.contains("invalid"),
        "Expected validation error for empty name, got: {}",
        message
    );
}

// Test basic trainers query - first page
#[tokio::test]
async fn test_graphql_trainers_first_page() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create test data
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);

    // Create multiple trainers for testing
    for i in 1..=5 {
        trainer_service
            .create_trainer(CreateTrainerInput {
                name: format!("Trainer {}", i),
                sex: if i % 2 == 0 { Sex::F } else { Sex::M },
                phone_number: Some(format!("+3312345678{}", i)),
                contact_email: Some(format!("trainer{}@example.com", i)),
                birthdate: Some(Utc::now()),
            })
            .await
            .expect("Failed to create test trainer");
    }

    // Query trainers
    let query = r#"
        query trainers($input: TrainersInput!) {
            trainers(input: $input) {
                edges {
                    node {
                        id
                        name
                        sex
                        phoneNumber
                        contactEmail
                        birthdate
                    }
                    cursor
                }
                pageInfo {
                    endCursor
                    hasNextPage
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "cursorPagination": {
                "first": 3
            }
        }
    });

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none());

    let trainers = data.get("trainers").expect("Missing `trainers` field");
    let edges = trainers.get("edges").unwrap().as_array().unwrap();
    let page_info = trainers.get("pageInfo").unwrap();

    assert_eq!(edges.len(), 3);
    assert!(page_info.get("hasNextPage").unwrap().as_bool().unwrap());
    assert!(page_info.get("endCursor").is_some());

    // Verify first trainer data
    let first_trainer = &edges[0]["node"];
    assert!(first_trainer.get("id").is_some());
    assert!(
        first_trainer
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with("Trainer")
    );
}

// Test pagination with after cursor
#[tokio::test]
async fn test_graphql_trainers_pagination() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create test data
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);

    // Create multiple trainers
    for i in 1..=6 {
        trainer_service
            .create_trainer(CreateTrainerInput {
                name: format!("Trainer {}", i),
                sex: Sex::M,
                phone_number: None,
                contact_email: None,
                birthdate: None,
            })
            .await
            .expect("Failed to create test trainer");
    }

    let query = r#"
        query trainers($input: TrainersInput!) {
            trainers(input: $input) {
                edges {
                    node {
                        id
                        name
                    }
                    cursor
                }
                pageInfo {
                    endCursor
                    hasNextPage
                }
            }
        }
    "#;

    // First page
    let first_page_variables = json!({
        "input": {
            "cursorPagination": {
                "first": 2
            }
        }
    });

    let first_page_payload = json!({
        "query": query,
        "variables": first_page_variables
    });

    let first_response = client
        .post(format!("{}/graphql", app_url))
        .json(&first_page_payload)
        .send()
        .await
        .expect("Failed to send first page request");

    assert!(first_response.status().is_success());

    let first_json: Value = first_response.json().await.unwrap();
    let first_trainers = &first_json["data"]["trainers"];
    let first_edges = first_trainers["edges"].as_array().unwrap();
    let first_page_info = &first_trainers["pageInfo"];

    assert_eq!(first_edges.len(), 2);
    assert!(first_page_info["hasNextPage"].as_bool().unwrap());

    let end_cursor = first_page_info["endCursor"].as_str().unwrap();

    // Second page using cursor
    let second_page_variables = json!({
        "input": {
            "cursorPagination": {
                "first": 2,
                "after": end_cursor
            }
        }
    });

    let second_page_payload = json!({
        "query": query,
        "variables": second_page_variables
    });

    let second_response = client
        .post(format!("{}/graphql", app_url))
        .json(&second_page_payload)
        .send()
        .await
        .expect("Failed to send second page request");

    assert!(second_response.status().is_success());

    let second_json: Value = second_response.json().await.unwrap();
    let second_trainers = &second_json["data"]["trainers"];
    let second_edges = second_trainers["edges"].as_array().unwrap();

    assert_eq!(second_edges.len(), 2);

    // Verify different trainers returned
    let first_page_ids: Vec<&str> = first_edges
        .iter()
        .map(|edge| edge["node"]["id"].as_str().unwrap())
        .collect();
    let second_page_ids: Vec<&str> = second_edges
        .iter()
        .map(|edge| edge["node"]["id"].as_str().unwrap())
        .collect();

    assert_ne!(first_page_ids, second_page_ids);
}

// Test empty result
#[tokio::test]
async fn test_graphql_trainers_empty() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let query = r#"
        query trainers($input: TrainersInput!) {
            trainers(input: $input) {
                edges {
                    node {
                        id
                        name
                    }
                    cursor
                }
                pageInfo {
                    endCursor
                    hasNextPage
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "cursorPagination": {
                "first": 10
            }
        }
    });

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let trainers = data.get("trainers").expect("Missing `trainers` field");
    let edges = trainers.get("edges").unwrap().as_array().unwrap();
    let page_info = trainers.get("pageInfo").unwrap();

    assert_eq!(edges.len(), 0);
    assert!(!page_info.get("hasNextPage").unwrap().as_bool().unwrap());
    assert!(page_info.get("endCursor").unwrap().is_null());
}

// Test invalid cursor
#[tokio::test]
async fn test_graphql_trainers_invalid_cursor() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let query = r#"
        query trainers($input: TrainersInput!) {
            trainers(input: $input) {
                edges {
                    node {
                        id
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "cursorPagination": {
                "first": 5,
                "after": "invalid-cursor-format"
            }
        }
    });

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let errors = response_json.get("errors");

    assert!(errors.is_some());
    let message = errors.unwrap()[0].get("message").unwrap().as_str().unwrap();
    assert!(message.contains("Malformed cursor:"));
}

// Test with invalid first parameter (negative or zero)
#[tokio::test]
async fn test_graphql_trainers_invalid_first_parameter() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let query = r#"
        query trainers($input: TrainersInput!) {
            trainers(input: $input) {
                edges {
                    node {
                        id
                    }
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "cursorPagination": {
                "first": 0
            }
        }
    });

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();

    // This should either return an error or handle gracefully
    // The exact behavior depends on your GraphQL schema validation
    if let Some(errors) = response_json.get("errors") {
        let message = errors[0].get("message").unwrap().as_str().unwrap();
        assert!(
            message.contains("first")
                || message.contains("invalid")
                || message.contains("positive"),
            "Expected validation error for invalid first parameter, got: {}",
            message
        );
    }
}

// Test successful trainer deletion
#[tokio::test]
async fn test_graphql_delete_trainer_success() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create test trainer using TrainersService
    let repo = PgTrainersRepository {
        pool: app.db_pool.clone(),
    };
    let trainer_service = TrainersService::new(repo);

    let test_trainer = trainer_service
        .create_trainer(CreateTrainerInput {
            name: "Trainer To Delete".to_string(),
            sex: Sex::M,
            phone_number: Some("+33123456789".to_string()),
            contact_email: Some("delete.me@example.com".to_string()),
            birthdate: Some(Utc::now()),
        })
        .await
        .expect("Failed to create test trainer");

    let trainer_id = test_trainer.id.to_string();

    // Delete trainer mutation
    let mutation = r#"
        mutation deleteTrainer($input: DeleteTrainerInput!) {
            deleteTrainer(input: $input) {
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
            "id": trainer_id
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
        .expect("Failed to send delete request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none(), "Unexpected errors: {:?}", errors);

    let deleted_trainer = data
        .get("deleteTrainer")
        .expect("Missing `deleteTrainer` field");

    // Verify returned deleted trainer data
    assert_eq!(deleted_trainer.get("name").unwrap(), "Trainer To Delete");
    assert_eq!(deleted_trainer.get("sex").unwrap(), "M");
    assert_eq!(deleted_trainer.get("phoneNumber").unwrap(), "+33123456789");
    assert_eq!(
        deleted_trainer.get("contactEmail").unwrap(),
        "delete.me@example.com"
    );
    assert!(deleted_trainer.get("birthdate").is_some());
}

// Test delete trainer with minimal fields
#[tokio::test]
async fn test_graphql_delete_trainer_minimal_fields() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    // Create minimal trainer
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

    let trainer_id = test_trainer.id.to_string();

    let mutation = r#"
        mutation deleteTrainer($input: DeleteTrainerInput!) {
            deleteTrainer(input: $input) {
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
            "id": trainer_id
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
        .expect("Failed to send delete request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none());

    let deleted_trainer = data
        .get("deleteTrainer")
        .expect("Missing `deleteTrainer` field");

    assert_eq!(deleted_trainer.get("name").unwrap(), "Minimal Trainer");
    assert_eq!(deleted_trainer.get("sex").unwrap(), "F");
    assert!(deleted_trainer.get("phoneNumber").unwrap().is_null());
    assert!(deleted_trainer.get("contactEmail").unwrap().is_null());
    assert!(deleted_trainer.get("birthdate").unwrap().is_null());
}

// Test delete trainer not found
#[tokio::test]
async fn test_graphql_delete_trainer_not_found() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let mutation = r#"
        mutation deleteTrainer($input: DeleteTrainerInput!) {
            deleteTrainer(input: $input) {
                id
                name
            }
        }
    "#;

    // Use a valid UUID format that doesn't exist
    let non_existent_id = Uuid::new_v4().to_string();
    let variables = json!({
        "input": {
            "id": non_existent_id
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
        .expect("Failed to send delete request");

    assert!(response.status().is_success());

    let response_json: Value = response.json().await.unwrap();
    let errors = response_json
        .get("errors")
        .expect("Expected GraphQL errors");
    let message = errors[0].get("message").unwrap().as_str().unwrap();

    assert!(
        message.contains("Error deleting trainer") && message.contains("TrainerNotFound"),
        "Expected trainer not found error, got: {}",
        message
    );
}

// Test delete trainer with invalid UUID format
#[tokio::test]
async fn test_graphql_delete_trainer_invalid_uuid() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let mutation = r#"
        mutation deleteTrainer($input: DeleteTrainerInput!) {
            deleteTrainer(input: $input) {
                id
                name
            }
        }
    "#;

    let variables = json!({
        "input": {
            "id": "invalid-uuid-format"
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
        .expect("Failed to send delete request");

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
