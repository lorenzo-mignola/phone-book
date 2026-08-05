use serde_json::{Value, json};

mod util;

#[tokio::test]
async fn should_get_all_contacts() {
    let server = util::setup_test().await;

    let response = server.get("/contacts").await;

    response.assert_status_ok();

    let body: Vec<Value> = response.json();
    assert_eq!(body.len(), 1);
}

#[tokio::test]
async fn should_get_contact() {
    let server = util::setup_test().await;

    let response = server.get("/contacts/1").await;

    response.assert_status_ok();

    response.assert_json_contains(&json!({
        "id": 1,
        "first_name": "test",
        "phone_numbers": ["+41 1234"]
    }));
}

#[tokio::test]
async fn should_return_404_when_not_found() {
    let server = util::setup_test().await;

    let response = server.get("/contacts/100").await;

    response.assert_status_not_found();
}
