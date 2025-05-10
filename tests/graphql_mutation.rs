use helpers::app_test::set_up_app_test;
use reqwest::Client;
use serde_json::Value;
use serde_json::json;

mod helpers;

#[tokio::test]
async fn test_graphql_register_dog() {
    let app_url = set_up_app_test().await;

    let client = Client::new();

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
        .post(app_url + "/graphql") // L'URL de votre endpoint GraphQL
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    println!("status: {}", response.status());
    assert!(
        response.status().is_success(),
        "Response status is not success"
    );

    let response_json: Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    let data = response_json.get("data").expect("Missing `data` field");
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
