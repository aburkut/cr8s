use crate::common::APP_HOST;
use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde_json::{Value, json};

pub mod common;

#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = common::create_test_crate(&client, &rustacean);

    assert_eq!(
        a_crate,
        json!({
                "id": a_crate["id"],
                "rustacean_id": rustacean["id"],
                "code": "Foo",
                "name": "Foo crate",
                "version": "0.1",
                "description": "Foo crate description",
                "created_at": a_crate["created_at"],
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crate() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let client = common::get_client_with_logged_in_viewer();
    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        a_crate,
        json!({
                "id": a_crate["id"],
                "rustacean_id": rustacean["id"],
                "code": "Foo",
                "name": "Foo crate",
                "version": "0.1",
                "description": "Foo crate description",
                "created_at": a_crate["created_at"],
        })
    );

    // Cleanup
    let client = common::get_client_with_logged_in_admin();
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crate_not_found() {
    // Setup
    let client = common::get_client_with_logged_in_admin();

    // Test
    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, 99999))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_get_crates() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate1 = common::create_test_crate(&client, &rustacean);
    let a_crate2 = common::create_test_crate(&client, &rustacean);

    // Test
    let client = common::get_client_with_logged_in_viewer();
    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate1));
    assert!(json.as_array().unwrap().contains(&a_crate2));

    // Clenup
    let client = common::get_client_with_logged_in_admin();
    common::delete_test_crate(&client, a_crate1);
    common::delete_test_crate(&client, a_crate2);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crates_with_logged_in_logged_out() {
    // Setup
    let client = Client::new();
    let client_logged_in = common::get_client_with_logged_in_admin();

    // Test
    let response = client.get(format!("{}/crates", APP_HOST)).send().unwrap();
    let response2 = client_logged_in
        .get(format!("{}/crates", APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response2.status(), StatusCode::OK);
}

#[test]
fn test_update_crate() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let rustacean2 = common::create_test_rustacean(&client);
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "fooz",
            "name": "Foo crate",
            "version": "0.2",
            "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
            "rustacean_id": rustacean2["id"],
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
                "id": a_crate["id"],
                "code": "fooz",
                "name": "Foo crate",
                "version": "0.2",
                "description": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum",
                "rustacean_id": rustacean2["id"],
                "created_at": a_crate["created_at"],
        })
    );

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_delete_crate() {
    // Setup
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}
