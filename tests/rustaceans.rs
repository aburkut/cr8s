use reqwest::{StatusCode, blocking::Client};
use serde_json::{Value, json};

pub mod common;

#[test]
fn test_get_rustaceans() {
    // Setup
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    // Test
    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // Cleanup
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    // Test
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo bar",
            "email": "foo@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    // Cleanup
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    // Test
    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Foo bar",
            "email": "foo@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    // Cleanup
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    // Test
    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .json(&json!({
            "name": "Fooz bar",
            "email": "fooz@bar.com",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Fooz bar",
            "email": "fooz@bar.com",
            "created_at": rustacean["created_at"],
        })
    );

    // Cleanup
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    // Test
    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    common::delete_test_rustacean(&client, rustacean);
}
