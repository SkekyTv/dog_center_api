use dog_center_api::entities::dogs::Dog;
use dog_center_api::infra::db::dogs_repository::PgDogsRepository;
use dog_center_api::shared::types::sex::Sex;
use dog_center_api::use_cases::dogs_service::DogsService;
use helpers::app_test::set_up_app_test;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json::Value;
use serde_json::json;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::sync::OnceCell;

mod helpers;

// TODO: Wrap these in helpers function
static APP_URL: OnceCell<String> = OnceCell::const_new();
static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

async fn get_app_url() -> &'static str {
    // Boot app once and store url
    APP_URL
        .get_or_init(|| async {
            let test_set_up = set_up_app_test().await;
            test_set_up.app_url
        })
        .await
}
async fn get_db_pool() -> &'static Pool<Postgres> {
    // Boot app once and store url
    DB_POOL
        .get_or_init(|| async {
            let test_set_up = set_up_app_test().await;
            test_set_up.db_pool
        })
        .await
}

#[tokio::test]
async fn test_graphql_register_dog_and_read_on_id() {
    let app_url = get_app_url().await;

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

#[tokio::test]
async fn query_dog() {
    let app_url = get_app_url().await;

    let client = Client::new();

    let pool = get_db_pool().await;
    let repo = PgDogsRepository { pool: pool.clone() };
    let dog_service = DogsService::new(repo);

    let dog = Dog::new(
        "panda".to_string(),
        Sex::F,
        None,
        ["caniche".to_string()].to_vec(),
        Some(4),
        None,
    );

    dog_service
        .create_dog(dog)
        .await
        .map_err(|e| panic!("Error creating dog: {}", e));

    let dog_id = "fake-id";

    let variables = json!({
        "input": {
            "id": dog_id
        }
    });
    let query = json!({
        "query": r#"
            query GetDog($input: DogInput!) {
                dog(input: $input) {
                    id
                    name
                    breed
                    age
                }
            }
        "#,
    });

    let payload = json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .get(format!("{}/graphql", app_url)) // L'URL de votre endpoint GraphQL
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), StatusCode::OK);

    assert!(
        response.status().is_success(),
        "Response status is not success"
    );
    let response_json: Value = match response.json().await {
        Ok(json) => json,
        Err(e) => panic!("panic: {}", e),
    };
    let data = response_json.get("data").expect("Missing `data` field");
    let errors = response_json.get("errors");

    assert!(errors.is_none());

    let dog = data.get("dog").expect("Missing `dog` field");

    assert_eq!(dog.get("name").unwrap(), "Pupuce");
    assert_eq!(dog.get("sex").unwrap(), "F");
    assert_eq!(dog.get("races").unwrap(), &json!(["Border Collie"]));
}
