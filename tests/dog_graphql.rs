use dog_center_api::entities::dogs::Dog;
use dog_center_api::infra::db::dogs_repository::PgDogsRepository;
use dog_center_api::shared::types::sex::Sex;
use dog_center_api::use_cases::dogs_service::CreateDogInput;
use dog_center_api::use_cases::dogs_service::DogsService;
use helpers::app_test::set_up_app_test;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json::Value;
use serde_json::json;

mod helpers;

// registerDog
#[tokio::test]
async fn test_graphql_register_dog() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();

    let client = Client::new();

    // Register Dog
    let mutation = r#"
        mutation registerDog($input: RegisterDogInput!) {
            registerDog(input: $input) {
                id
                name
                sex
                races
            }
        }
    "#;

    let variables = json!({
        "input": {
            "name": "Pupuce",
            "races": ["Border Collie"],
            "sex": "F"
        }
    });

    let payload = json!({
        "query": mutation,
        "variables": variables
    });

    let response = client
        .post(format!("{}/graphql", app_url)) // L'URL de votre endpoint GraphQL
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

    let register_dog = data
        .get("registerDog")
        .expect("Missing `registerDog` field");

    assert_eq!(register_dog.get("name").unwrap(), "Pupuce");
    assert_eq!(register_dog.get("sex").unwrap(), "F");
    assert_eq!(
        register_dog.get("races").unwrap(),
        &json!(["Border Collie"])
    );
}

// query dog
#[tokio::test]
async fn test_graphql_query_dog() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let pool = app.db_pool.clone();

    let client = Client::new();

    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    let dog = dog_service
        .create_dog(CreateDogInput {
            name: "panda".to_string(),
            sex: Sex::F,
            birthdate: None,
            races: ["caniche".to_string()].to_vec(),
            weight: vec![4],
            icad_id: None,
        })
        .await
        .unwrap();

    let dog_id = dog.id;

    let variables = json!({
        "input": {
            "id": dog_id
        }
    });
    let query = r#"
            query GetDog($input: DogInput!) {
                dog(input: $input) {
                    id
                    name
                    birthdate
                    sex
                    races
                    weight {
                        unit
                        value
                    }
                  }
            }
        "#;

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
    let status = response.status();
    let text = response.text().await.unwrap_or_default();

    if !status.is_success() {
        panic!("Response status is not success: {}\nBody: {}", status, text);
    }

    // Ensuite, si tu veux parser le JSON :
    let response_json: Value = serde_json::from_str(&text).expect("Failed to parse JSON response");

    assert_eq!(status, StatusCode::OK);

    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none());

    let dog = data.get("dog").expect("Missing `dog` field");

    assert_eq!(dog.get("name").unwrap(), "panda");
    assert_eq!(dog.get("sex").unwrap(), "F");
    assert_eq!(dog.get("races").unwrap(), &json!(["caniche"]));
    assert_eq!(
        dog.get("weight").unwrap(),
        &json!([{"unit":"G", "value":4}])
    );
}

// dog update
#[tokio::test]
async fn test_graphql_update_dog_name_only() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let pool = app.db_pool.clone();
    let client = Client::new();

    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    let dog = dog_service
        .create_dog(CreateDogInput {
            name: "Rex".to_string(),
            sex: Sex::M,
            birthdate: None,
            races: vec!["Labrador".to_string()],
            weight: vec![10],
            icad_id: None,
        })
        .await
        .expect("Dog creation failed");

    let dog_id = dog.id;

    let mutation = r#"
        mutation updateDog($input: UpdateDogInput!) {
            updateDog(input: $input) {
                id
                name
                sex
                races
            }
        }
    "#;

    let variables = json!({
        "input": {
            "id": dog_id,
            "name": "Rexy"
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
    let errors = body.get("errors");
    if let Some(e) = errors {
        panic!("GraphQL returned errors: {}", e);
    }

    let updated = body
        .pointer("/data/updateDog")
        .expect("Missing `updateDog`");

    assert_eq!(updated.get("name").unwrap(), "Rexy");
    assert_eq!(updated.get("sex").unwrap(), "M");
    assert_eq!(updated.get("races").unwrap(), &json!(["Labrador"]));
}

#[tokio::test]
async fn test_graphql_update_dog_not_found() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let fake_id = uuid::Uuid::new_v4();

    let mutation = r#"
        mutation updateDog($input: UpdateDogInput!) {
            updateDog(input: $input) {
                id
                name
            }
        }
    "#;

    let variables = json!({
        "input": {
            "id": fake_id,
            "name": "Ghost"
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
        message.contains("not found") || message.contains("NotFound"),
        "Expected 'not found' error, got: {}",
        message
    );
}

// dog activation status
#[tokio::test]
async fn test_graphql_toggle_dog_activation_status_desactivate() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let pool = app.db_pool.clone();
    let client = Client::new();

    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    let dog = dog_service
        .create_dog(CreateDogInput {
            name: "Rex".to_string(),
            sex: Sex::M,
            birthdate: None,
            races: vec!["Labrador".to_string()],
            weight: vec![10],
            icad_id: None,
        })
        .await
        .expect("Dog creation failed");

    let dog_id = dog.id;

    let mutation = r#"
        mutation toggleDogActivationStatus($input: ToggleDogActivationStatusInput!) {
            toggleDogActivationStatus(input: $input) {
                id
                desactivationStatus
            }
        }
        "#;

    let variables = json!({
        "input": {
            "id": dog_id,
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
    let errors = body.get("errors");
    if let Some(e) = errors {
        panic!("GraphQL returned errors: {}", e);
    }

    let updated = body
        .pointer("/data/toggleDogActivationStatus")
        .expect("Missing `toggleDogActivationStatus`");

    assert_eq!(updated.get("desactivationStatus").unwrap(), &json!(true));
}

#[tokio::test]
async fn test_graphql_toggle_dog_activation_status_reactivate() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let pool = app.db_pool.clone();
    let client = Client::new();

    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    let input_dog = CreateDogInput {
        name: "Rex".to_string(),
        sex: Sex::M,
        birthdate: None,
        races: vec!["Labrador".to_string()],
        weight: vec![10],
        icad_id: None,
    };

    let dog = dog_service
        .create_dog(input_dog.clone())
        .await
        .expect("Dog creation failed");

    let toogled_dog = dog_service
        .toggle_activation_status(dog.id)
        .await
        .expect("Dog toggle activation failed");

    assert!(toogled_dog.desactivated_at.is_some());

    let dog_id = dog.id;

    let mutation = r#"
        mutation toggleDogActivationStatus($input: ToggleDogActivationStatusInput!) {
            toggleDogActivationStatus(input: $input) {
                id
                desactivationStatus
            }
        }
        "#;

    let variables = json!({
        "input": {
            "id": dog_id,
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
    let errors = body.get("errors");
    if let Some(e) = errors {
        panic!("GraphQL returned errors: {}", e);
    }

    let updated = body
        .pointer("/data/toggleDogActivationStatus")
        .expect("Missing `toggleDogActivationStatus`");

    assert_eq!(updated.get("desactivationStatus").unwrap(), &json!(false));
}

#[tokio::test]
async fn test_graphql_toggle_dog_activation_status_not_found() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let client = Client::new();

    let fake_id = uuid::Uuid::new_v4();

    let mutation = r#"
        mutation toggleDogActivationStatus($input: ToggleDogActivationStatusInput!) {
            toggleDogActivationStatus(input: $input) {
                id
                desactivationStatus
            }
        }
        "#;

    let variables = json!({
        "input": {
            "id": fake_id,
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
        message.contains("not found") || message.contains("NotFound"),
        "Expected 'not found' error, got: {}",
        message
    );
}

// Dogs
#[tokio::test]
async fn test_graphql_query_dogs_pagination() {
    let app = set_up_app_test().await;
    let app_url = app.app_url.clone();
    let pool = app.db_pool.clone();
    let client = Client::new();

    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    // Création de 3 chiens
    let input_dogs = vec![
        CreateDogInput {
            name: "Alpha".to_string(),
            sex: Sex::M,
            birthdate: None,
            races: vec!["Shiba".to_string()],
            weight: vec![10],
            icad_id: None,
        },
        CreateDogInput {
            name: "Bravo".to_string(),
            sex: Sex::F,
            birthdate: None,
            races: vec!["Beagle".to_string()],
            weight: vec![8],
            icad_id: None,
        },
        CreateDogInput {
            name: "Charlie".to_string(),
            sex: Sex::M,
            birthdate: None,
            races: vec!["Poodle".to_string()],
            weight: vec![6],
            icad_id: None,
        },
    ];

    let mut dogs: Vec<Dog> = vec![];
    for input_dog in &input_dogs {
        let dog = dog_service
            .create_dog(input_dog.clone())
            .await
            .expect("Dog creation failed");
        dogs.push(dog);
    }

    // Requête GraphQL page 1
    let query = r#"
        query ListDogs($input: DogsInput!) {
            dogs(input: $input) {
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
                "first": 2
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
        .expect("Failed to send first page request");

    assert!(response.status().is_success());

    let body: Value = response.json().await.expect("Invalid JSON response");

    let dogs_data = body.pointer("/data/dogs").expect("Missing `dogs`");
    let edges = dogs_data
        .get("edges")
        .expect("Missing `edges`")
        .as_array()
        .unwrap();
    let page_info = dogs_data.get("pageInfo").expect("Missing `pageInfo`");

    assert_eq!(edges.len(), 2);
    assert_eq!(page_info.get("hasNextPage").unwrap(), &json!(true));

    let end_cursor = page_info
        .get("endCursor")
        .expect("Missing `endCursor`")
        .as_str()
        .expect("endCursor should be string");

    // Requête GraphQL page 2
    let variables_page2 = json!({
        "input": {
            "cursorPagination": {
                "first": 2,
                "after": end_cursor
            }
        }
    });

    let payload_page2 = json!({
        "query": query,
        "variables": variables_page2
    });

    let response_page2 = client
        .post(format!("{}/graphql", app_url))
        .json(&payload_page2)
        .send()
        .await
        .expect("Failed to send second page request");

    assert!(response_page2.status().is_success());

    let body_page2: Value = response_page2
        .json()
        .await
        .expect("Invalid JSON response for page 2");

    let dogs_data_page2 = body_page2.pointer("/data/dogs").expect("Missing `dogs`");
    let edges_page2 = dogs_data_page2
        .get("edges")
        .expect("Missing `edges`")
        .as_array()
        .unwrap();
    let page_info_page2 = dogs_data_page2.get("pageInfo").expect("Missing `pageInfo`");

    assert_eq!(edges_page2.len(), 1);
    assert_eq!(page_info_page2.get("hasNextPage").unwrap(), &json!(false));
}
